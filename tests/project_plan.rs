use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn plan(root: &Path, target: &str, options: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_vibe"))
        .arg("--root")
        .arg(root)
        .arg("--config")
        .arg(root.join("must-not-create.toml"))
        .args(["project", "plan", target])
        .args(options)
        .output()
        .unwrap()
}

fn json(output: std::process::Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn plans_all_profiles_without_creating_target_or_config() {
    let root = tempfile::tempdir().unwrap();
    for (profile, source) in [
        ("software", "code-repo/src"),
        ("onec-designer", "code-repo/src/configuration"),
        ("onec-extension", "code-repo/src/extensions"),
        ("onec-edt", "code-repo/src/edt"),
    ] {
        let result = json(plan(
            root.path(),
            "new",
            &["--mode", "new", "--profile", profile],
        ));
        assert_eq!(result["schema"], "vibe.project-plan.v1");
        assert_eq!(result["executable"], false);
        assert_eq!(result["profile"], profile);
        assert!(result["directories"]
            .as_array()
            .unwrap()
            .contains(&Value::from(source)));
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    }
}

#[test]
fn existing_checkout_is_only_inspected() {
    let root = tempfile::tempdir().unwrap();
    let checkout = root.path().join("checkout");
    let repo = git2::Repository::init(&checkout).unwrap();
    fs::create_dir(checkout.join("nested")).unwrap();
    fs::write(checkout.join("README.md"), b"existing contents").unwrap();
    let head = fs::read(repo.path().join("HEAD")).unwrap();
    let config = fs::read(repo.path().join("config")).unwrap();

    let result = json(plan(root.path(), "checkout", &["--mode", "attach"]));
    assert_eq!(result["directories"], serde_json::json!([]));
    assert_eq!(result["files"], serde_json::json!([]));
    assert!(!plan(root.path(), "checkout", &["--mode", "new"])
        .status
        .success());
    assert!(!plan(root.path(), "checkout/nested", &["--mode", "attach"])
        .status
        .success());
    assert_eq!(
        fs::read(checkout.join("README.md")).unwrap(),
        b"existing contents"
    );
    assert_eq!(fs::read(repo.path().join("HEAD")).unwrap(), head);
    assert_eq!(fs::read(repo.path().join("config")).unwrap(), config);
    assert!(!root.path().join("must-not-create.toml").exists());
}

#[test]
fn rejects_escape_and_non_checkout() {
    let root = tempfile::tempdir().unwrap();
    assert!(!plan(root.path(), "../escape", &["--mode", "new"])
        .status
        .success());
    let absolute = root.path().join("absolute");
    assert!(
        !plan(root.path(), absolute.to_str().unwrap(), &["--mode", "new"])
            .status
            .success()
    );
    fs::create_dir(root.path().join("plain")).unwrap();
    assert!(!plan(root.path(), "plain", &["--mode", "attach"])
        .status
        .success());
}

#[cfg(unix)]
#[test]
fn rejects_symlink_parent_and_dangling_target() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    symlink(outside.path(), root.path().join("link")).unwrap();
    symlink(outside.path().join("missing"), root.path().join("dangling")).unwrap();
    assert!(!plan(root.path(), "link/new", &["--mode", "new"])
        .status
        .success());
    assert!(!plan(root.path(), "dangling", &["--mode", "new"])
        .status
        .success());
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
}

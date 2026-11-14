use anyhow::{ensure, Context, Result};
use clap::{Subcommand, ValueEnum};
use serde::Serialize;
use std::{
    fs,
    io::{self, Write},
    path::{Component, Path, PathBuf},
};

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// Print a non-executable JSON preview; no files or Git state are changed
    Plan {
        /// Target path relative to the explicit --root
        target: PathBuf,
        #[arg(long, value_enum)]
        mode: Mode,
        /// Source layout for a new project (default: software); not used for attach
        #[arg(long, value_enum)]
        profile: Option<Profile>,
    },
}

#[derive(Clone, Copy, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    New,
    Attach,
}

#[derive(Clone, Copy, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    Software,
    OnecDesigner,
    OnecExtension,
    OnecEdt,
}

#[derive(Serialize)]
struct Plan {
    schema: &'static str,
    executable: bool,
    root: PathBuf,
    target: PathBuf,
    mode: Mode,
    profile: Option<Profile>,
    directories: Vec<&'static str>,
    files: Vec<&'static str>,
}

impl ProjectCommand {
    pub fn run(&self, root: Option<&Path>) -> Result<()> {
        let Self::Plan {
            target,
            mode,
            profile,
        } = self;
        let root = root.context("project plan requires an explicit --root")?;
        let plan = build_plan(root, target, *mode, *profile)?;
        let mut out = io::stdout().lock();
        serde_json::to_writer_pretty(&mut out, &plan)?;
        writeln!(out)?;
        Ok(())
    }
}

fn build_plan(root: &Path, target: &Path, mode: Mode, profile: Option<Profile>) -> Result<Plan> {
    ensure!(
        target.components().next().is_some()
            && target
                .components()
                .all(|c| matches!(c, Component::Normal(_))),
        "target must be a non-empty relative path inside --root"
    );
    let root = root.canonicalize().context("resolve workspace root")?;
    ensure!(root.is_dir(), "workspace root must be a directory");

    let mut path = root.clone();
    let mut parts = target.components().peekable();
    while let Some(part) = parts.next() {
        path.push(part.as_os_str());
        let new_leaf = parts.peek().is_none() && matches!(mode, Mode::New);
        match fs::symlink_metadata(&path) {
            Ok(meta) => {
                ensure!(
                    !meta.file_type().is_symlink(),
                    "symlink in target path: {}",
                    path.display()
                );
                ensure!(!new_leaf, "new target already exists: {}", path.display());
                ensure!(meta.is_dir(), "not a directory: {}", path.display());
                let resolved = path.canonicalize().context("resolve target component")?;
                ensure!(
                    resolved.starts_with(&root) && resolved != root,
                    "target escapes workspace root"
                );
            }
            Err(error) if new_leaf && error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error).with_context(|| format!("inspect {}", path.display())),
        }
    }

    let mut plan = Plan {
        schema: "vibe.project-plan.v1",
        executable: false,
        root,
        target: path,
        mode,
        profile,
        directories: Vec::new(),
        files: Vec::new(),
    };
    match mode {
        Mode::New => {
            let profile = profile.unwrap_or(Profile::Software);
            plan.profile = Some(profile);
            plan.directories.extend([
                "docs-repo/config",
                "docs-repo/docs",
                "docs-repo/docops",
                "docs-repo/docsecops",
                "docs-repo/AI/knowledge",
                "docs-repo/AI/runbooks",
                "docs-repo/AI/skills",
                "docs-repo/knowledge-kit",
                "code-repo/tests",
            ]);
            let source = match profile {
                Profile::Software => "code-repo/src",
                Profile::OnecDesigner => "code-repo/src/configuration",
                Profile::OnecExtension => "code-repo/src/extensions",
                Profile::OnecEdt => "code-repo/src/edt",
            };
            plan.directories.push(source);
            if !matches!(profile, Profile::Software) {
                plan.directories.push("code-repo/scripts");
            }
            plan.files.extend([
                "docs-repo/config/fup-projects.json",
                "docs-repo/config/fup-context.json",
                "docs-repo/README.md",
                "docs-repo/AI/README.md",
            ]);
        }
        Mode::Attach => {
            ensure!(profile.is_none(), "--profile is only valid with --mode new");
            let repo = git2::Repository::open_ext(
                &plan.target,
                git2::RepositoryOpenFlags::NO_SEARCH,
                std::iter::empty::<&Path>(),
            )
            .context("target must be an existing Git checkout")?;
            ensure!(
                repo.workdir().map(Path::canonicalize).transpose()?.as_ref()
                    == Some(&plan.target.canonicalize()?),
                "target must be the root of a non-bare Git checkout"
            );
        }
    }
    Ok(plan)
}

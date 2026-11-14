# Local project plan

```sh
vibe --root /path/to/workspace project plan new-project --mode new
vibe --root /path/to/workspace project plan checkout --mode attach
```

`--root` must be an existing directory. The target is relative to it;
intermediate directories must exist, and symlinks below the root are rejected.
New mode requires an absent target and supports `software` (default),
`onec-designer`, `onec-extension`, and `onec-edt`. The preview shows the
intended layout and entry files, not a full list of SDK-generated internals.
Attach mode requires the root of a non-bare Git checkout and proposes no
filesystem changes to it. The command prints JSON without initializing
workspace configuration, creating directories, or applying any SDK operation.

## DonJuan SDK/tools handoff

The `plan` command is read-only. Its JSON object
(`vibe.project-plan.v1`, `executable: false`) is a local draft, not an approval.
It contains absolute `root` and `target`, `mode`, nullable `profile`, and
`directories` / `files` relative to the target. The SDK must independently
validate the root, derive the layout and recheck the filesystem before writing.

Agreed local 0.9 boundary, not a native MCP contract: invoke a trusted Python
entry point with fixed argv (no shell). The root and executable come from
trusted local configuration, not from the draft.

| SDK verb | JSON input on stdin |
| --- | --- |
| `prepare` | The `vibe.project-plan.v1` object directly |
| `apply` | `{"operation_key":"...","plan_digest":"..."}` |
| `readback` | `{"operation_key":"..."}` |

The separate owner-only bridge is:

```sh
vibe --root ROOT donjuan-operation --python ABS_PYTHON --sdk-root ABS_SDK VERB
```

It invokes `ABS_PYTHON -E -B ABS_SDK/scripts/donjuan_project_adapter.py --root ROOT VERB`.
Install a compatible DonJuan SDK separately. Use an existing workspace root
outside Git checkouts, controlled by the local operator.
SDK stdout uses `donjuan.project-operation.v1` with `status`
(`prepared`, `applied`, `partial`, `absent`, or `denied`) and optional
`operation_key`, `plan_digest`, `plan`, `result`, and `reason`. Exit 2 is a
denial; normal responses, including absent or partial, use exit 0.

Local apply is an explicit owner action under OS permissions against the
exact SDK digest. A digest does not grant access. Following a lost reply,
read back the same operation key before any retry. SDK alone writes project
records; attach does not write the existing checkout. No remote provisioning,
ACL change, or Znatok operation is part of this contract.

# DonJuan workspace setup

Status: proposed. The existing MIT Rust fork is the code repository.
DonJuan retains one documentation repository per project; this file describes
the integration contract, not a second journal writer.

## Repository layout

```text
vibe-workspace (code fork)
  src/       upstream CLI, wizard and MCP
  tests/     integration and regression fixtures
  ci/        pinned native build recipes
  docs/      developer guides and this integration specification
  LICENSE    upstream MIT license

generated workspace/
  docs-repo/
    config/           project routing and repository UID references
    README.md        managed project entry point
    docs/            requirements, architecture and decisions
    docops/          build, delivery and operations guides
    docsecops/       access and security
    AI/              project knowledge, runbooks and skills
    knowledge-kit/   SDK journals, registry and local state
  code-repo/
    src/              selected profile only
    tests/
```

## Wizard contract

This layout applies only to new DonJuan projects. Do not move or rewrite
existing projects, documentation paths, or CMDB records.
New projects opt in through documentation.root = "." and the four root
sections in CMDB. Executable deployment recipes stay in the code repository
or a dedicated delivery repository; docops links to them.

1. Ask for project name, purpose, location and source profile. Show the plan
   before applying it. Reject an existing destination; never rewrite it.
2. Invoke the pinned DonJuan Python SDK initializer as a typed command, without
   shell interpolation. Do not reproduce the UID or journal writer in Rust.
3. Return the project UID, both roots, check result and limitations. Do not
   assume a local registry entry grants service access. Remote provisioning,
   thread assignment and team permissions are separate owner-approved steps.
4. If initialization fails, report the exact destination and leave it available
   for inspection. Never silently reuse or delete a partial workspace.

## Source profiles

| Profile | Source layout | Gate |
| --- | --- | --- |
| Software | `src/`, `tests/` | Clean-checkout tests on the selected platform |
| 1C Designer configuration | `src/configuration/`, optional `src/extensions/<name>/` | Native Designer load/check/export |
| 1C Designer extension | `src/extensions/<name>/` only | Native load/check against a pinned compatible base |
| 1C EDT | `src/edt/` containing the actual EDT project | Pinned EDT/platform, native import/export checks |

Fresh/ACM uses Designer XML/BSL by default. Preserve native object UUIDs, forms,
layouts and resources. Do not generate fake native metadata or install blanket
binary exclusions. CF/CFE/CFU and database dumps do not belong in source.
Use the Fresh source-management runbook and repository-specific policy before
applying ignore rules or moving between Designer and EDT.
An actual Designer export includes Configuration.xml and all dependent resources.
An actual EDT export retains .project, project settings and its own src tree.
For CFE pin the exact base configuration version. In a clean native checkout,
check source completeness, load into an isolated base, verify Thin/Server and
WebClient as applicable, export CF/CFE and independently load and test it.
Record source SHA, platform version, job ID, artifact SHA-256 and test outcomes.

## Acceptance and delivery

- All four profiles initialize synthetic workspaces and pass the SDK guard.
- Existing paths and foreign project data remain unchanged.
- A fresh Codex thread can discover its permitted project through the local
  adapter; a foreign thread is denied. Test OpenClaw separately.
- macOS first: native signed packaging and clean-machine install. Windows
  VM9011 has its own native checks; Linux checks are independent.
- Publication: protected Git ref → Semaphore → pinned DEV build and tests →
  SHA-linked artifact → delivery → feedback. Do not treat local smoke tests as
  native CI or activation evidence.

Not implemented by this specification: Rust wizard invocation, remote creation
and grants, native 1C source generation, Windows-native Python SDK execution
(`fcntl` currently requires a POSIX environment), or Semaphore activation.
The current SDK still needs a legacy component selector for project-scoped TASK
creation. The initializer does not invent new subprojects; removing that
requirement is a separate SDK migration.

# Workspace MCP + FUP adapter candidate

Upstream: toolprint/vibe-workspace, MIT, commit
`8dc3f8c88f1765c8113041f437c33f83d71d4aec`, Cargo version0.0.12,
pinned Rust toolchain1.88.0. Preserve upstream licence and history.

This directory is a source review and portable contract candidate. No runtime,
shared tool exposure, host configuration, permissions or build lane is admitted.
The catalogue enumerates all30 registered tools; classification is a proposal,
not proof of a tool's safety or a user's right to call it.

## MVP boundary

The central Go MCP owns trusted caller identity, project approvals, provider
bindings and CI/feedback correlation. A local Go adapter should manage registered
repository roots and typed Git actions. The existing Rust CLI can remain a
reviewed helper while the MVP is delivered. Replacing Rust is not a prerequisite.
Use unary calls for bounded plans/actions, server streaming for status, and
bidirectional gRPC only where cancellation, acknowledgements and resume need it.

Profiles are data, never authorization. The synthetic FUP contracts cannot enable
execution or supply thread identity, grants, commands, credentials or disk paths.
The existing FUP transactional writer remains the only journal writer. A future
adapter calls its approved capture API with trusted host identity; it must not
create a second project database or repair projections automatically. A generic
client name in a schema does not establish native authentication support.

## Blocking source findings

* All30 upstream tools register without a per-caller/project policy context.
* `exec_git_command` accepts arbitrary Git arguments. The current implementation
  splits whitespace and invokes Git directly, not a shell; Git configuration,
  hooks, credential helpers and destructive operations still require controls.
* `WorkspaceManager::execute_command` logs individual failures but returns Ok,
  allowing its MCP handler to report success. Native reproduction and a tested
  fix are pending. CLI output also needs separation from MCP protocol stdout.
* Reset/restore/cleanup, app launch and dependency installation need distinct
  explicit plans, not automatic activation from recovered configuration.
* The upstream MCP documentation reports an initialize-version issue. It has
  not been reproduced here; do not patch agent-host configuration as a workaround.

## Admission and delivery

Resolve the real repository and caller-owned permissions first. Prepare a bounded
plan tied to registered root, input hashes and generation; preserve dirty files
and use CAS on apply. Never expose raw shell/Git commands through the gateway.
Unknown write ACKs require reading the same operation/ref, not another push.

Normal delivery is local owning Git -> remote review branch -> Semaphore ->
assigned admitted DEV -> artifact SHA -> scoped delivery -> feedback. DEV is a
pipeline variable. Windows and macOS require independently accepted builders;
no local workstation or production build fallback is implied.

Only reviewed synthetic contracts and their tests are included. No private
project history, real UIDs, credentials, telemetry or local paths are imported.
Python contract tests are auxiliary checks, not the new MCP runtime. Static tests
do not establish Rust compilation, installed protocol compatibility or execution.

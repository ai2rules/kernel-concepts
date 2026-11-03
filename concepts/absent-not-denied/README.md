# Absent, not denied

A deny-list names what is forbidden and lets everything else through. A world names what
exists. A tool the world does not declare is never shown to the model, and a call that
names it anyway gets `Absent`: there is nothing to call. The tests show the difference
on a tool nobody thought to list (`purge_repo_v2`).

```bash
cargo test -p absent-not-denied
```

## What this leaves out

The kernel compiles its world from a YAML manifest, and existence there is richer than a
set of names: data classes, scoped capabilities (an action that exists only for some
paths or arguments), shell commands classified into actions, and a tool surface projected
per principal for MCP servers. How a host reacts to an absent call (refuse, or ask) is
the host adapter's setting, not the world's. None of that is here.

## The real behaviour

`kit/one-kernel/cases.yaml`: `unknown_tool_is_absent` and
`unprojected_jira_delete_is_absent` (an MCP tool the upstream offers but the world does
not declare: absent, and not even listed).

# Monotonic taint

Once a session has read something untrusted, everything the model proposes afterwards
may be steered by it. The session is marked tainted, and the mark only goes up: there is
no operation that cleans it. Outbound calls are refused in a tainted session, including
honest ones; that cost is the design, not a bug.

```bash
cargo test -p monotonic-taint
```

## What this leaves out

In the kernel, what taints is declared by the world (per action, and per path for path
scope), the post-call taint is part of every verdict, and which actions refuse under
taint, ask, or are unaffected is the world's choice. Background sessions turn some
questions into refusals. Flow contracts add a second axis, confidentiality, so a
private value can be kept from a public sink even in a clean session. Here there is one
bit and one rule.

## The real behaviour

`kit/one-kernel/cases.yaml`: `clean_jira_read_is_allowed_and_taints`,
`tainted_jira_comment_is_denied`, `tainted_curl_is_denied` and
`post_call_taint_never_decreases`. For the second axis, `kit/one-kernel/flow-cases.yaml`.

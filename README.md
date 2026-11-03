# ai2rules kernel concepts

Public companion to the [ai2rules](https://ai2rules.dev) kernel. The kernel's source is
private; the kernel itself is released on npm as
[`ai2rules-harness`](https://www.npmjs.com/package/ai2rules-harness). This repository
holds two things, and they do different jobs:

| | What | Use it to |
|---|---|---|
| [`kit/`](kit/) + [`conformance/`](conformance/) | The kernel's own cases — a gate request and the verdict the kernel gives it — and a runner that sends each one to the **released binary** | **check** a claim about what the kernel decides |
| [`concepts/`](concepts/) | Small Rust illustrations of the core ideas | **understand** an idea |

The rule between them: **the concepts explain, the conformance kit proves.** Concept
code is a few hundred lines written to be read. It is not the kernel, it is not
hardened, and it must not be used as a security control. Every concept says what it
leaves out and which conformance cases show the real behaviour.

## Check the kernel yourself

```bash
npm install -g ai2rules-harness@$(sed 's/^v//' KERNEL_VERSION)   # the released binary
cargo run -p conformance                                          # 45 cases, about a second
```

`KERNEL_VERSION` names the kernel release the kit comes from; install the npm release of
the same version. Each case in [`kit/one-kernel/`](kit/one-kernel/) is a request and the
expected `decision`, `rule`, post-call `taint` (and `flow`, for flow contracts). The
runner prints one line per case and fails on any difference. You can also send a single
case by hand:

```bash
echo '{"tool":"frobnicate_the_flux","arguments":{},"context":{"taint":"clean","mode":"interactive","source_channel":"user_prompt","usage":{}}}' \
  | harness gate --world kit/one-kernel/demo-world.yaml
# {"v":1,"decision":"ABSENT","rule":"unknown_to_ontology",...}
```

Three worlds:

- `demo-world.yaml` with `cases.yaml`: absent tools, taint, shell command classes,
  instruction-file writes under taint;
- `roots-world.yaml` with `roots-cases.yaml`: path scope; the runner builds the
  directory the cases describe (Unix only, it needs a symlink);
- `flow-world.yaml` with `flow-cases.yaml`: typed flow contracts, where a private value
  may not reach a public sink.

[`kit/benchmark/`](kit/benchmark/) has the MCP governance benchmark's three scenarios,
the world the ai2rules target runs with, and the results of that release. The runner
and the intentionally weak reference gateway it is compared with are not published, so
the scenarios can be read and the results checked for consistency, but not rerun here.

## Concepts

| Concept | Idea | Real behaviour |
|---|---|---|
| [`absent-not-denied`](concepts/absent-not-denied/) | A tool the world does not declare is not refused; it does not exist | `unknown_tool_is_absent`, `unprojected_jira_delete_is_absent` |
| [`monotonic-taint`](concepts/monotonic-taint/) | After an untrusted read, the session stays tainted; outbound calls are refused | `clean_jira_read_is_allowed_and_taints`, `tainted_jira_comment_is_denied`, `post_call_taint_never_decreases` |
| [`sealed-spec`](concepts/sealed-spec/) | Only a checked, sealed value reaches the executor; the type is the proof | internal structure, not visible through the gate (see its README) |

```bash
cargo test --workspace
```

## Versions and updates

The cases live in the kernel and are never edited here. Each kernel release attaches
them as `harness-conformance-kit.tar.gz`; `scripts/update-kit.sh <version>` replaces
`kit/` and `KERNEL_VERSION` (it needs access to the private kernel repository, so the
maintainer runs it). CI runs the kit against the npm binary on every change and weekly.

## Security

To report a vulnerability in the released kernel, see [SECURITY.md](SECURITY.md).

## Licence

`MIT OR Apache-2.0`, at your option: [LICENSE-MIT](LICENSE-MIT),
[LICENSE-APACHE](LICENSE-APACHE). The concept code is for reading and learning; it is
provided as is and is not a security control.

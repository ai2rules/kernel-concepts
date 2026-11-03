# ai2rules conformance kit — kernel v0.8.1

- `one-kernel/`: three worlds and their cases. Each case is a gate request with the
  verdict the kernel gives it (`decision`, `rule`, post-call `taint`, and `flow`
  where present). Send `request` to `harness gate --world <world>` and compare.
  In the roots pair, `{project}` stands for a real directory laid out as the
  cases describe.
- `benchmark/`: the MCP governance benchmark's scenarios, the world and upstream
  catalogue the ai2rules target runs with, and the results this release produced.
  The runner and the weak reference gateway are not included.

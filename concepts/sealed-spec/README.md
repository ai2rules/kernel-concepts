# Only a sealed spec crosses

The model proposes; it never executes. A proposal is checked against the world and, if it
passes, becomes an `ExecutionSpec`, a type with private fields and no public
constructor. The executor takes only that type. Holding a spec is proof the check ran,
and a code path that hands the executor a raw proposal does not compile (the crate's
`compile_fail` doctest shows it).

```bash
cargo test -p sealed-spec
```

## What this leaves out

In the kernel the check is the whole decision: the compiled world, taint, path scope,
approvals and budgets, and the spec carries what execution may touch. The executor is a
separate crate that never imports the kernel, so decisions and execution cannot be mixed
by accident. Here the check is a name and an argument shape.

## The real behaviour

No conformance case shows this directly: it is a property of how the kernel's code is
built, not of a verdict. What is visible from outside is that a refused call never runs.
The verdicts in `kit/` are what `harness gate` returns before anything executes. The
architecture is described on the site, under [Kernel documents](https://ai2rules.dev/kernel/).

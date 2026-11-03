# Security

## Reporting a vulnerability

This repository is the public place to report a security problem in the ai2rules kernel
(`ai2rules-harness` on npm), including in its released behaviour shown by the cases in
`kit/`.

Use GitHub's private vulnerability reporting on **this** repository: the **Security**
tab → **Report a vulnerability**. That opens a private thread with the maintainer.
Please do not open a public issue with the details.

Include the `ai2rules-harness` version (`harness --version`), the world and the request
that show the problem, and what you expected instead.

## Scope

- **In scope:** the released `ai2rules-harness` binary and its verdicts.
- **Not in scope:** the code in `concepts/`. It is illustrative, deliberately incomplete,
  and not a security control; a gap in it is not a vulnerability. If a concept
  misrepresents what the kernel does, an ordinary issue is welcome.

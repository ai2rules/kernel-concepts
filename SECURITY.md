# Security

## Reporting a vulnerability

This repository is the public place to report a security problem in the ai2rules kernel
(`ai2rules-harness` on npm), including in its released behaviour shown by the cases in
`kit/`.

Use GitHub's private vulnerability reporting on **this** repository: the **Security**
tab → **Report a vulnerability**. That opens a private thread with the maintainer.
Please do not open a public issue with the details.

If private reporting is unavailable to you, open an ordinary issue here saying only
that you have a security report and asking for a private channel — no details in the
issue.

Include the `ai2rules-harness` version (`harness --version`), the world and the request
that show the problem, and what you expected instead.

## Scope

- **In scope:** anything that makes the released kernel's verdict wrong,
  unenforced or bypassable; the `ai2rules-harness` npm packages and the published
  binaries; the release path that builds them; and anything that lets the harness
  modify, disable or point away from the policy that governs it. Some properties are
  out of scope by design (the hook is advisory, not a sandbox); if unsure, report
  privately and we will say which it is.
- **Design arguments** — that a known property *should* change — are welcome as
  ordinary issues here.
- **Not in scope:** the code in `concepts/`. It is illustrative, deliberately incomplete,
  and not a security control; a gap in it is not a vulnerability. If a concept
  misrepresents what the kernel does, an ordinary issue is welcome.

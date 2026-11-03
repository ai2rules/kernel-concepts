//! # Only a sealed spec crosses
//!
//! The model proposes; it never executes. Its proposal is checked against the world
//! and, if it passes, turned into an [`ExecutionSpec`]. The executor accepts nothing
//! else. `ExecutionSpec` has private fields and no public constructor, so the only
//! way to hold one is to have gone through [`build`]: the type itself is the proof
//! that the check ran.
//!
//! This one cannot be broken by a missed `if` in the executor, because there is no
//! code path that hands the executor a raw proposal. The compiler refuses it:
//!
//! ```compile_fail
//! use sealed_spec::{ExecutionSpec, execute};
//! // Forging a spec outside the module does not compile: the fields are private.
//! let spec = ExecutionSpec { program: "rm".into(), args: vec!["-rf".into(), "/".into()] };
//! execute(&spec);
//! ```
//!
//! **An illustration, not the kernel.** See this crate's README for what it leaves
//! out and for the conformance case that shows the real behaviour.

/// What the model proposes: untrusted, unchecked.
#[derive(Debug, Clone)]
pub struct Proposal {
    pub program: String,
    pub args: Vec<String>,
}

/// Whether a program accepts these arguments.
type ArgCheck = fn(&[String]) -> bool;

/// The world's list of programs that exist, and the argument shapes each allows.
pub struct World {
    programs: Vec<(&'static str, ArgCheck)>,
}

impl World {
    /// A tiny example world: `ls` with any arguments, `git` only for `status`/`log`.
    pub fn example() -> Self {
        Self {
            programs: vec![
                ("ls", |_| true),
                ("git", |args| {
                    matches!(args.first().map(String::as_str), Some("status" | "log"))
                }),
            ],
        }
    }
}

/// Why a proposal did not become a spec.
#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The program does not exist in this world.
    Absent,
    /// The program exists; these arguments are not allowed.
    ArgumentsRejected,
}

/// A checked, executable call. Private fields, no public constructor: only
/// [`build`] makes one.
#[derive(Debug, PartialEq, Eq)]
pub struct ExecutionSpec {
    program: String,
    args: Vec<String>,
}

impl ExecutionSpec {
    pub fn program(&self) -> &str {
        &self.program
    }
    pub fn args(&self) -> &[String] {
        &self.args
    }
}

/// The only door from a proposal to something the executor will take.
pub fn build(world: &World, proposal: &Proposal) -> Result<ExecutionSpec, Refusal> {
    let (_, allowed) = world
        .programs
        .iter()
        .find(|(name, _)| *name == proposal.program)
        .ok_or(Refusal::Absent)?;
    if !allowed(&proposal.args) {
        return Err(Refusal::ArgumentsRejected);
    }
    Ok(ExecutionSpec {
        program: proposal.program.clone(),
        args: proposal.args.clone(),
    })
}

/// The executor. It takes a spec, never a proposal. (Here it only describes what
/// it would run.)
pub fn execute(spec: &ExecutionSpec) -> String {
    format!("would run: {} {}", spec.program, spec.args.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(program: &str, args: &[&str]) -> Proposal {
        Proposal {
            program: program.into(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn a_checked_proposal_becomes_a_spec_and_runs() {
        let spec = build(&World::example(), &p("git", &["status"])).unwrap();
        assert_eq!(execute(&spec), "would run: git status");
    }

    #[test]
    fn a_program_the_world_lacks_never_becomes_a_spec() {
        assert_eq!(
            build(&World::example(), &p("curl", &["evil.example"])),
            Err(Refusal::Absent)
        );
    }

    #[test]
    fn allowed_program_with_disallowed_arguments_is_refused() {
        assert_eq!(
            build(&World::example(), &p("git", &["push", "--force"])),
            Err(Refusal::ArgumentsRejected)
        );
    }
}

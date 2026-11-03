//! # Absent, not denied
//!
//! A deny-list names what the agent may not do, and lets everything else through.
//! A world names what exists. A tool the world does not declare is not refused: it
//! is never offered to the model, and a call that names it anyway is `Absent` —
//! there is nothing to call.
//!
//! The difference shows the first time a tool appears that nobody thought of. The
//! deny-list lets it through, because it is not on the list. The world does not have
//! it, so it does not exist.
//!
//! **An illustration, not the kernel.** See this crate's README for what it leaves
//! out and for the conformance case that shows the real behaviour.

use std::collections::BTreeMap;

/// What a world says about a call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Allow,
    /// The tool exists but a human has to agree first.
    Ask,
    /// The tool exists; this call is refused.
    Deny,
    /// The tool does not exist in this world.
    Absent,
}

/// How a declared action is disposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    Allow,
    Ask,
}

/// A world: the actions that exist, and how each is disposed.
#[derive(Debug, Default)]
pub struct World {
    actions: BTreeMap<String, Disposition>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn declare(mut self, action: &str, disposition: Disposition) -> Self {
        self.actions.insert(action.to_string(), disposition);
        self
    }

    /// The tool surface the model is shown: what exists, nothing else.
    pub fn project(&self) -> Vec<&str> {
        self.actions.keys().map(String::as_str).collect()
    }

    /// Judge a call by name. Anything not declared is absent.
    pub fn decide(&self, tool: &str) -> Verdict {
        match self.actions.get(tool) {
            None => Verdict::Absent,
            Some(Disposition::Allow) => Verdict::Allow,
            Some(Disposition::Ask) => Verdict::Ask,
        }
    }
}

/// The contrast: a list of forbidden names, everything else allowed.
#[derive(Debug, Default)]
pub struct DenyList {
    denied: Vec<String>,
}

impl DenyList {
    pub fn new(denied: &[&str]) -> Self {
        Self {
            denied: denied.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn decide(&self, tool: &str) -> Verdict {
        if self.denied.iter().any(|d| d == tool) {
            Verdict::Deny
        } else {
            Verdict::Allow
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> World {
        World::new()
            .declare("read_file", Disposition::Allow)
            .declare("write_file", Disposition::Ask)
    }

    #[test]
    fn the_model_is_shown_only_what_exists() {
        assert_eq!(world().project(), vec!["read_file", "write_file"]);
    }

    #[test]
    fn a_declared_tool_is_judged_by_its_disposition() {
        assert_eq!(world().decide("read_file"), Verdict::Allow);
        assert_eq!(world().decide("write_file"), Verdict::Ask);
    }

    #[test]
    fn a_tool_nobody_thought_of_gets_through_a_deny_list() {
        let deny = DenyList::new(&["delete_repo", "drop_database"]);
        assert_eq!(deny.decide("delete_repo"), Verdict::Deny);
        // A new, equally destructive tool, under a name the list does not have:
        assert_eq!(deny.decide("purge_repo_v2"), Verdict::Allow);
    }

    #[test]
    fn the_same_tool_does_not_exist_in_a_world() {
        assert_eq!(world().decide("purge_repo_v2"), Verdict::Absent);
        assert!(!world().project().contains(&"purge_repo_v2"));
    }
}

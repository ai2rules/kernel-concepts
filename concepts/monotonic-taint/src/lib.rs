//! # Monotonic taint
//!
//! Once a session has read something untrusted — a web page, an issue comment, a
//! file from an inbox — everything the model proposes afterwards may be steered by
//! it. So the session is marked tainted, and the mark only goes one way: there is
//! no operation that makes a tainted session clean again. Tools that would let
//! steered output leave the machine are refused in a tainted session.
//!
//! The cost is real and deliberate: after one untrusted read, a harmless outbound
//! call is refused too. The kernel does not try to tell steered calls from honest
//! ones, because that is the judgement a model cannot be trusted to make.
//!
//! **An illustration, not the kernel.** See this crate's README for what it leaves
//! out and for the conformance case that shows the real behaviour.

/// Whether untrusted content has entered the session. `Clean < Tainted`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Taint {
    #[default]
    Clean,
    Tainted,
}

impl Taint {
    /// The only way to combine taints: the higher one wins.
    pub fn join(self, other: Taint) -> Taint {
        self.max(other)
    }
}

/// Where a read came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// The user, or the project's own files.
    Trusted,
    /// The web, an issue, an inbox: anything someone else can write.
    Untrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Allow,
    Deny,
}

/// A session. Its taint can be raised and read, never lowered: there is no setter.
#[derive(Debug, Default)]
pub struct Session {
    taint: Taint,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn taint(&self) -> Taint {
        self.taint
    }

    /// Record a read. An untrusted read taints the session for good.
    pub fn read(&mut self, source: Source) {
        let t = match source {
            Source::Trusted => Taint::Clean,
            Source::Untrusted => Taint::Tainted,
        };
        self.taint = self.taint.join(t);
    }

    /// Judge an outbound call (a web request, a comment, a push).
    pub fn outbound(&self) -> Verdict {
        match self.taint() {
            Taint::Clean => Verdict::Allow,
            Taint::Tainted => Verdict::Deny,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_clean_session_may_reach_out() {
        let mut s = Session::new();
        s.read(Source::Trusted);
        assert_eq!(s.outbound(), Verdict::Allow);
    }

    #[test]
    fn one_untrusted_read_and_outbound_calls_are_refused() {
        let mut s = Session::new();
        s.read(Source::Untrusted);
        assert_eq!(s.outbound(), Verdict::Deny);
    }

    #[test]
    fn trusted_reads_afterwards_do_not_clean_it() {
        let mut s = Session::new();
        s.read(Source::Untrusted);
        for _ in 0..10 {
            s.read(Source::Trusted);
        }
        assert_eq!(s.taint(), Taint::Tainted);
        assert_eq!(s.outbound(), Verdict::Deny);
    }

    #[test]
    fn the_order_of_reads_does_not_matter() {
        let orders = [
            [Source::Untrusted, Source::Trusted],
            [Source::Trusted, Source::Untrusted],
        ];
        for order in orders {
            let mut s = Session::new();
            for src in order {
                s.read(src);
            }
            assert_eq!(s.taint(), Taint::Tainted);
        }
    }

    #[test]
    fn join_never_goes_down() {
        for a in [Taint::Clean, Taint::Tainted] {
            for b in [Taint::Clean, Taint::Tainted] {
                assert!(a.join(b) >= a && a.join(b) >= b);
            }
        }
    }
}

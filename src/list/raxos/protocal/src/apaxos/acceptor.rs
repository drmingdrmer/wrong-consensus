use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;

use validit::Validate;

use crate::apaxos::branch::Branch;
use crate::apaxos::branch::HEAD_UNDECIDED;
use crate::apaxos::decided::Decided;
use crate::apaxos::history::History;
use crate::Types;

#[derive(Clone, Default, Debug)]
pub struct Acceptor<T: Types> {
    /// A Time that is smaller than any one of these time
    /// is not allow to commit.
    pub forbidden_commit_time: HashSet<T::Time>,

    pub history: T::History,
}

impl<T: Types> Validate for Acceptor<T> {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl<T: Types> Acceptor<T> {
    /// Handle the phase-1 request from a [`Proposer`], i.e., set up a new
    /// [`Time`] point.
    ///
    /// Returns a [`Branch`] for appending new event if the given time is
    /// allowed to commit a new event at.
    /// Otherwise, returns the [`Time`] that disables this proposal.
    pub(crate) fn handle_phase1_request(
        &mut self,
        commit_time: T::Time,
    ) -> Result<Branch<T, { HEAD_UNDECIDED }>, T::Time> {
        self.check_committable(&commit_time)?;

        self.forbid_smaller_commit_time(commit_time);

        Ok(self.history.branch(commit_time))
    }

    pub(crate) fn handle_phase2_request(&mut self, decided: Decided<T>) -> Result<(), T::Time> {
        dbg!("handle_phase2_request", &decided);

        let head_time = decided.head_time();
        self.check_committable(&head_time)?;

        self.history.merge(decided.into_history());

        Ok(())
    }

    fn forbid_smaller_commit_time(&mut self, time: T::Time) {
        self.forbidden_commit_time.insert(time);

        for t in self.forbidden_commit_time.clone().iter() {
            if t < &time {
                self.forbidden_commit_time.remove(t);
            }
        }
    }

    /// Check it is allowed to commit at the specified time.
    fn check_committable(&self, time: &T::Time) -> Result<(), T::Time> {
        for t in &self.forbidden_commit_time {
            if t > time {
                return Err(*t);
            }
        }

        Ok(())
    }
}

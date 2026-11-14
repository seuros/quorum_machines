#![no_std]
#![doc = include_str!("../README.md")]

use core::fmt;

/// Why a call changed nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The index names no seat of the membership.
    UnknownMember,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnknownMember => "the index names no seat of the membership",
        })
    }
}

impl core::error::Error for Refusal {}

/// What an honoured [`Quorum::acknowledge`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Acknowledgement {
    /// The member had not acknowledged this round; the count went up by one.
    Counted,
    /// The member had already acknowledged this round; nothing changed.
    Duplicate,
}

/// A distinct-voter threshold tracker over `members` seats, at most `N`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quorum<const N: usize> {
    members: usize,
    required: usize,
    seen: [bool; N],
    count: usize,
}

impl<const N: usize> Quorum<N> {
    /// Most seats a membership of this type can hold.
    pub const CAPACITY: usize = N;

    /// Construct a tracker over seats `0..members` that is reached by
    /// `required` distinct acknowledgements.
    ///
    /// `None` unless `1 <= required <= members <= N`.
    #[must_use]
    pub const fn new(members: usize, required: usize) -> Option<Self> {
        if required == 0 || required > members || members > N {
            return None;
        }
        Some(Self {
            members,
            required,
            seen: [false; N],
            count: 0,
        })
    }

    /// Construct a tracker reached by strictly more than half of `members`.
    ///
    /// `None` unless `1 <= members <= N`.
    #[must_use]
    pub const fn majority(members: usize) -> Option<Self> {
        Self::new(members, members / 2 + 1)
    }

    /// Record a member acknowledgement.
    ///
    /// # Errors
    ///
    /// [`Refusal::UnknownMember`] when `member` is not below
    /// [`Self::members`]; nothing changed.
    pub fn acknowledge(&mut self, member: usize) -> Result<Acknowledgement, Refusal> {
        // One check, so no arm exists that the roster bound already ruled out:
        // a seat is known when it is on the roster (`members <= N`).
        let on_roster = member < self.members;
        let Some(seen) = self.seen.get_mut(member).filter(|_| on_roster) else {
            return Err(Refusal::UnknownMember);
        };
        if *seen {
            return Ok(Acknowledgement::Duplicate);
        }
        *seen = true;
        self.count += 1;
        Ok(Acknowledgement::Counted)
    }

    /// Whether `member` has acknowledged this round. `false` off the roster.
    #[must_use]
    pub fn acknowledged(&self, member: usize) -> bool {
        member < self.members && self.seen.get(member).is_some_and(|seen| *seen)
    }

    /// Whether enough distinct members have acknowledged.
    #[must_use]
    pub const fn reached(&self) -> bool {
        self.count >= self.required
    }

    /// Number of distinct acknowledgements.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Number of seats in the membership.
    #[must_use]
    pub const fn members(&self) -> usize {
        self.members
    }

    /// Distinct acknowledgements that reach the quorum.
    #[must_use]
    pub const fn required(&self) -> usize {
        self.required
    }

    /// Start another round with the same membership and threshold.
    pub const fn reset(&mut self) {
        self.seen = [false; N];
        self.count = 0;
    }
}

/// Whether two nonempty thresholds necessarily intersect within one membership.
#[must_use]
pub const fn intersects(members: usize, reads: usize, writes: usize) -> bool {
    reads > 0 && writes > 0 && reads <= members && writes <= members && reads > members - writes
}

#[cfg(test)]
mod tests;

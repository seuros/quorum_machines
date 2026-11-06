#![no_std]
#![doc = include_str!("../README.md")]

/// A fixed-membership, distinct-voter threshold tracker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quorum<const N: usize> {
    required: usize,
    seen: [bool; N],
    count: usize,
}

impl<const N: usize> Quorum<N> {
    /// Construct a tracker, rejecting zero or impossible thresholds.
    #[must_use]
    pub const fn new(required: usize) -> Option<Self> {
        if required == 0 || required > N {
            return None;
        }
        Some(Self {
            required,
            seen: [false; N],
            count: 0,
        })
    }

    /// Construct a majority tracker; an empty membership is invalid.
    #[must_use]
    pub const fn majority() -> Option<Self> {
        Self::new(N / 2 + 1)
    }

    /// Record a member acknowledgement. `false` means an invalid index.
    #[must_use]
    pub fn acknowledge(&mut self, member: usize) -> bool {
        let Some(seen) = self.seen.get_mut(member) else {
            return false;
        };
        if !*seen {
            *seen = true;
            self.count += 1;
        }
        true
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

    /// Start another round with the same fixed membership and threshold.
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

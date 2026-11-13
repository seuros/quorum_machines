//! Randomized comparison against a set-of-members reference.
use crate::{Acknowledgement, Quorum, Refusal};

/// `SplitMix64`: deterministic draws, no dependency.
struct Draws(u64);

impl Draws {
    const fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut x = self.0;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        x ^ (x >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        usize::try_from(self.next() % (bound as u64)).expect("fits")
    }
}

const CAPACITY: usize = 16;

#[test]
fn random_rounds_match_a_distinct_member_reference() {
    let mut draws = Draws(4216);
    for _ in 0..500 {
        let members = 1 + draws.below(CAPACITY);
        let required = 1 + draws.below(members);
        let mut quorum = Quorum::<CAPACITY>::new(members, required).expect("valid");
        let mut seen = [false; CAPACITY];
        for _ in 0..200 {
            if draws.below(40) == 0 {
                quorum.reset();
                seen = [false; CAPACITY];
            }
            let member = draws.below(CAPACITY + 4);
            let before = quorum.clone();
            let answer = quorum.acknowledge(member);
            if member >= members {
                assert_eq!(answer, Err(Refusal::UnknownMember));
                assert_eq!(quorum, before);
            } else if seen[member] {
                assert_eq!(answer, Ok(Acknowledgement::Duplicate));
                assert_eq!(quorum, before);
            } else {
                assert_eq!(answer, Ok(Acknowledgement::Counted));
                seen[member] = true;
                assert_eq!(quorum.count(), before.count() + 1);
            }
            let count = seen.iter().filter(|seen| **seen).count();
            assert_eq!(quorum.count(), count);
            assert_eq!(quorum.reached(), count >= required);
            for (index, seen) in seen.iter().enumerate() {
                assert_eq!(quorum.acknowledged(index), *seen);
            }
        }
    }
}

//! Randomized comparison against a set-of-members reference.
use crate::{Acknowledgement, Quorum, Refusal};
use matryona_machines::testing::{SplitMix64, refuses, unchanged};

/// A draw in `0..bound`.
fn below(draws: &mut SplitMix64, bound: usize) -> usize {
    usize::try_from(draws.below(bound as u64)).expect("fits")
}

const CAPACITY: usize = 16;

#[test]
fn random_rounds_match_a_distinct_member_reference() {
    let mut draws = SplitMix64::new(4216);
    for _ in 0..500 {
        let members = 1 + below(&mut draws, CAPACITY);
        let required = 1 + below(&mut draws, members);
        let mut quorum = Quorum::<CAPACITY>::new(members, required).expect("valid");
        let mut seen = [false; CAPACITY];
        for _ in 0..200 {
            if below(&mut draws, 40) == 0 {
                quorum.reset();
                seen = [false; CAPACITY];
            }
            let member = below(&mut draws, CAPACITY + 4);
            if member >= members {
                refuses(&mut quorum, Refusal::UnknownMember, |q| {
                    q.acknowledge(member)
                });
            } else if seen[member] {
                assert_eq!(
                    unchanged(&mut quorum, |q| q.acknowledge(member)),
                    Ok(Acknowledgement::Duplicate)
                );
            } else {
                let before = quorum.count();
                assert_eq!(quorum.acknowledge(member), Ok(Acknowledgement::Counted));
                seen[member] = true;
                assert_eq!(quorum.count(), before + 1);
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

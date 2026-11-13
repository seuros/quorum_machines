#![allow(clippy::expect_used)]
mod model;

use super::*;

#[test]
fn distinct_members_not_messages_reach_majority() {
    let mut quorum = Quorum::<5>::majority(5).expect("valid quorum");
    assert_eq!(quorum.acknowledge(0), Ok(Acknowledgement::Counted));
    let once = quorum.clone();
    for _ in 0..10 {
        assert_eq!(quorum.acknowledge(0), Ok(Acknowledgement::Duplicate));
        assert_eq!(quorum, once);
    }
    assert_eq!(quorum.count(), 1);
    assert!(!quorum.reached());
    assert_eq!(quorum.acknowledge(2), Ok(Acknowledgement::Counted));
    assert_eq!(quorum.acknowledge(4), Ok(Acknowledgement::Counted));
    assert!(quorum.reached());
    assert!(quorum.acknowledged(4));
    assert!(!quorum.acknowledged(1));
    assert!(!quorum.acknowledged(5));
    assert_eq!(quorum.count(), 3);
}

#[test]
fn an_unknown_member_is_refused_and_changes_nothing() {
    let mut quorum = Quorum::<5>::majority(5).expect("valid quorum");
    assert_eq!(quorum.acknowledge(1), Ok(Acknowledgement::Counted));
    let before = quorum.clone();
    for member in [5, 6, usize::MAX] {
        assert_eq!(quorum.acknowledge(member), Err(Refusal::UnknownMember));
        assert_eq!(quorum, before);
    }
}

/// The roster is the membership, not the compiled capacity: a seat the
/// capacity could hold but the membership does not have is refused.
#[test]
fn a_seat_past_the_membership_is_unknown_even_within_capacity() {
    let mut quorum = Quorum::<64>::new(3, 2).expect("valid quorum");
    assert_eq!((quorum.members(), quorum.required()), (3, 2));
    assert_eq!(Quorum::<64>::CAPACITY, 64);
    let before = quorum.clone();
    assert_eq!(quorum.acknowledge(3), Err(Refusal::UnknownMember));
    assert_eq!(quorum.acknowledge(63), Err(Refusal::UnknownMember));
    assert_eq!(quorum, before);
    assert_eq!(quorum.acknowledge(2), Ok(Acknowledgement::Counted));
    assert_eq!(Quorum::<64>::majority(3).map(|q| q.required()), Some(2));
    assert_eq!(Quorum::<64>::majority(64).map(|q| q.required()), Some(33));
}

#[test]
fn thresholds_and_intersection_handle_boundaries() {
    assert!(Quorum::<0>::majority(0).is_none());
    assert!(Quorum::<3>::majority(0).is_none());
    assert!(Quorum::<3>::majority(4).is_none());
    assert!(Quorum::<3>::new(3, 0).is_none());
    assert!(Quorum::<3>::new(3, 4).is_none());
    assert!(Quorum::<3>::new(2, 3).is_none());
    assert!(Quorum::<3>::new(4, 2).is_none());
    assert!(Quorum::<3>::new(usize::MAX, 1).is_none());
    assert!(Quorum::<3>::new(3, 3).is_some());
    assert!(intersects(5, 3, 3));
    assert!(!intersects(5, 2, 3));
    assert!(!intersects(0, 0, 0));
    assert!(intersects(usize::MAX, usize::MAX, 1));
}

#[test]
fn a_new_round_has_no_previous_acknowledgements() {
    let mut quorum = Quorum::<3>::majority(3).expect("membership");
    assert_eq!(quorum.acknowledge(0), Ok(Acknowledgement::Counted));
    assert_eq!(quorum.acknowledge(1), Ok(Acknowledgement::Counted));
    assert!(quorum.reached());
    quorum.reset();
    assert_eq!(quorum, Quorum::<3>::majority(3).expect("membership"));
    assert_eq!(quorum.acknowledge(1), Ok(Acknowledgement::Counted));
    assert!(!quorum.reached());
}

#[test]
fn refusals_describe_themselves() {
    extern crate std;
    use std::string::ToString;
    let error: &dyn core::error::Error = &Refusal::UnknownMember;
    assert_ne!(error.to_string(), "");
}

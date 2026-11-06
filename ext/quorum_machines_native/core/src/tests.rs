#![allow(clippy::expect_used)]
use super::*;

#[test]
fn distinct_members_not_messages_reach_majority() {
    let mut quorum = Quorum::<5>::majority().expect("valid quorum");
    for _ in 0..10 {
        assert!(quorum.acknowledge(0));
    }
    assert_eq!(quorum.count(), 1);
    assert!(!quorum.reached());
    assert!(quorum.acknowledge(2));
    assert!(quorum.acknowledge(4));
    assert!(quorum.reached());
    assert!(!quorum.acknowledge(5));
    assert_eq!(quorum.count(), 3);
}

#[test]
fn thresholds_and_intersection_handle_boundaries() {
    assert!(Quorum::<0>::majority().is_none());
    assert!(Quorum::<3>::new(0).is_none());
    assert!(Quorum::<3>::new(4).is_none());
    assert!(intersects(5, 3, 3));
    assert!(!intersects(5, 2, 3));
    assert!(!intersects(0, 0, 0));
    assert!(intersects(usize::MAX, usize::MAX, 1));
}

#[test]
fn a_new_round_has_no_previous_acknowledgements() {
    let mut quorum = Quorum::<3>::majority().expect("membership");
    assert!(quorum.acknowledge(0));
    assert!(quorum.acknowledge(1));
    assert!(quorum.reached());
    quorum.reset();
    assert_eq!(quorum.count(), 0);
    assert!(!quorum.reached());
    assert!(quorum.acknowledge(2));
    assert!(!quorum.reached());
}

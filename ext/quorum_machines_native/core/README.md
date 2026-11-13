# quorum-machines

> **AMS Gifford**, NCC-4216, *Gifford*-class quorum relay.

Engine-room flight manual: count distinct bridge acknowledgements toward a threshold.

Allocation-free, `no_std` quorum tracking over fixed membership indices.
Create a fresh tracker, or `reset` one, for each operation, term, or membership epoch.

`Quorum<N>` holds a membership of up to `N` seats, numbered from zero.
`Quorum::new(members, required)` builds a tracker over seats `0..members` that
is reached by `required` distinct acknowledgements; `Quorum::majority(members)`
asks for strictly more than half. `intersects` checks threshold arithmetic
only: read/write intersection does not by itself prove consistency, freshness,
or consensus safety. Membership identity, authentication, epoch matching, and
durable acknowledgements are the consumer's responsibility. Joint-consensus
membership changes are not implemented.

```rust
use quorum_machines::{Acknowledgement, Quorum, Refusal};
# fn example() -> Option<()> {
let mut votes = Quorum::<8>::majority(3)?;
assert_eq!(votes.acknowledge(0), Ok(Acknowledgement::Counted));
assert_eq!(votes.acknowledge(0), Ok(Acknowledgement::Duplicate));
assert!(!votes.reached());
assert_eq!(votes.acknowledge(3), Err(Refusal::UnknownMember));
assert_eq!(votes.acknowledge(1), Ok(Acknowledgement::Counted));
assert!(votes.reached());
# Some(())
# }
# assert_eq!(example(), Some(()));
```

## Contract

- **Capacity and membership.** `N` is the compiled capacity
  (`Quorum::CAPACITY`); `members` is the roster of this tracker, fixed at
  construction. `new` and `majority` are constructors that validate
  configuration and answer `None` unless `1 <= required <= members <= N`.
- **Refusals.** `Refusal` is the one refusal type and has one variant:
  `UnknownMember`, the index is not below `members()`. A seat the capacity
  could hold but the membership does not have is unknown too. A refusal
  changes nothing: the tracker compares equal to a clone taken before the call.
- **Repeats are honoured.** `acknowledge` answers `Ok(Acknowledgement)`:
  `Counted` when the member is new this round, `Duplicate` when that member
  had already acknowledged. The tracker keeps one bit per seat, so it can
  prove the repeat; a `Duplicate` changes nothing. `acknowledged(member)`
  reads the same bit.
- **Rounds.** `reset` clears every seat and is always honoured. After it a
  member that acknowledged the previous round is `Counted` again: the tracker
  keeps no history across rounds, and telling an old round's acknowledgement
  from a new one is the caller's job.
- **Queries.** `reached`, `count`, `members`, `required`, `acknowledged` and
  `intersects` are pure and answer plain values.
- **No ids, no time.** Member indices are supplied by the caller; the tracker
  issues nothing and reads no clock.

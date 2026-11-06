# quorum-machines

> **AMS Gifford**, NCC-4216, *Gifford*-class quorum relay.

Engine-room flight manual: count distinct bridge acknowledgements toward a threshold.

Allocation-free, `no_std` quorum tracking over fixed membership indices.
Duplicate acknowledgements never increase the count. Invalid indices are rejected.
Create a fresh tracker for each operation, term, or membership epoch.

`Quorum::majority()` uses strictly more than half of the fixed membership.
`Quorum::new(required)` supports other positive thresholds up to the membership
size. `intersects` checks threshold arithmetic only: read/write intersection
does not by itself prove consistency, freshness, or consensus safety. Membership
identity, authentication, epoch matching, and durable acknowledgements are the
consumer's responsibility. Joint-consensus membership changes are not implemented.

```rust
use quorum_machines::Quorum;
# fn example() -> Option<()> {
let mut votes = Quorum::<3>::majority()?;
assert!(votes.acknowledge(0));
assert!(!votes.reached());
assert!(votes.acknowledge(1));
assert!(votes.reached());
# Some(())
# }
# assert_eq!(example(), Some(()));
```

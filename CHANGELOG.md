# Changelog

## [0.3.2](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.3.1...quorum_machines/v0.3.2) (2026-11-14)


### Bug Fixes

* **deps:** require matryona-machines &gt;= 0.4 ([ee5959f](https://github.com/seuros/quorum_machines/commit/ee5959fb73211bb6e1220683299496cc35540e55))

## [0.3.1](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.3.0...quorum_machines/v0.3.1) (2026-11-14)


### Bug Fixes

* **deps:** require matryona-machines &gt;= 0.3 ([772b947](https://github.com/seuros/quorum_machines/commit/772b947f38374d5a70fd9dc6558d6351ec3d95fc))

## [0.3.0](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.2.1...quorum_machines/v0.3.0) (2026-11-14)


### ⚠ BREAKING CHANGES

* adopt matryona-machines 0.2 (symbols! tables)

### Features

* adopt matryona-machines 0.2 (symbols! tables) ([a5d4b53](https://github.com/seuros/quorum_machines/commit/a5d4b5369c873d4759130496269333b9474d9c86))

## [0.2.1](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.2.0...quorum_machines/v0.2.1) (2026-11-14)


### Bug Fixes

* name the offending argument in conversion errors and compact quorum tests ([dbb792b](https://github.com/seuros/quorum_machines/commit/dbb792baccabb3f64264f0b89b94fcff48b64537))

## [0.2.0](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.1.4...quorum_machines/v0.2.0) (2026-11-13)


### ⚠ BREAKING CHANGES

* converted to the fleet conventions. One exhaustive Refusal per crate, Result from every operation that can refuse, a refusal changes nothing, provable idempotent repeats are honoured, Ruby mirrors the core. No compatibility aliases. Quorum::new(members, required) and majority(members); a duplicate ack says so.

### Features

* answer acknowledge with Result&lt;Acknowledgement, Refusal&gt; and move the roster into the core ([4c018ca](https://github.com/seuros/quorum_machines/commit/4c018caa889d01bf9f8d111e264ae9681c182d9a))

## [0.1.4](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.1.3...quorum_machines/v0.1.4) (2026-11-06)


### Bug Fixes

* crates.io keywords and categories for the core ([e44f52f](https://github.com/seuros/quorum_machines/commit/e44f52ff6146189773c81ab45182ea3e535e600b))

## [0.1.3](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.1.2...quorum_machines/v0.1.3) (2026-11-06)


### Bug Fixes

* keep test sources out of the packaged gem and crate ([0157188](https://github.com/seuros/quorum_machines/commit/0157188143615bee19c5c09578df1dc1cae368c0))

## [0.1.2](https://github.com/seuros/quorum_machines/compare/quorum_machines/v0.1.1...quorum_machines/v0.1.2) (2026-11-06)


### Bug Fixes

* cross builds no longer compile for the host ([084aeb6](https://github.com/seuros/quorum_machines/commit/084aeb6fe3749ff30cc6322e2fd00514c7bb3716))

## [0.1.1](https://github.com/seuros/quorum_machines/compare/quorum_machines-v0.1.0...quorum_machines/v0.1.1) (2026-11-06)


### Features

* initial extraction ([9513e8e](https://github.com/seuros/quorum_machines/commit/9513e8e339474618d9d028df80fdacc1b7d8c1f7))

use magnus::typed_data::Obj;
use magnus::{Error, Ruby, Symbol, Value, function, method, prelude::*};
use matryona_machines::ruby::{Symbols, integer};
use quorum_machines::{Acknowledgement, Quorum, Refusal};
use std::cell::RefCell;

/// Seats compiled into the binding: the core's `N`.
const CAPACITY: usize = 64;

type Core = Quorum<CAPACITY>;

matryona_machines::symbols! {
    /// The core's refusals as Symbols: the snake case of each variant.
    Refusals for Refusal as "a refusal" {
        UnknownMember => "unknown_member",
    }
}

matryona_machines::symbols! {
    /// The core's honoured `acknowledge` answers as Symbols.
    Acknowledgements for Acknowledgement as "an acknowledgement" {
        Counted => "counted",
        Duplicate => "duplicate",
    }
}

#[magnus::wrap(class = "QuorumMachines::Quorum", free_immediately, size)]
struct Tracker(RefCell<Core>);

impl Tracker {
    fn build(ruby: &Ruby, core: Option<Core>) -> Result<Self, Error> {
        core.map(|core| Self(RefCell::new(core))).ok_or_else(|| {
            Error::new(
                ruby.exception_range_error(),
                format!("members and required must satisfy 1 <= required <= members <= {CAPACITY}"),
            )
        })
    }

    fn new(ruby: &Ruby, members: Value, required: Value) -> Result<Self, Error> {
        let members = integer(members, "members")?;
        let required = integer(required, "required")?;
        Self::build(ruby, Core::new(members, required))
    }

    fn majority(ruby: &Ruby, members: Value) -> Result<Self, Error> {
        let members = integer(members, "members")?;
        Self::build(ruby, Core::majority(members))
    }

    fn acknowledge(ruby: &Ruby, this: Obj<Self>, member: Value) -> Result<Symbol, Error> {
        this.check_frozen()?;
        // Every conversion happens before the core is asked.
        let member = integer(member, "member")?;
        let outcome = this.0.borrow_mut().acknowledge(member);
        Ok(match outcome {
            Ok(acknowledgement) => Acknowledgements::symbol(ruby, acknowledgement),
            Err(refusal) => Refusals::symbol(ruby, refusal),
        })
    }

    fn acknowledged(this: &Self, member: Value) -> Result<bool, Error> {
        let member = integer(member, "member")?;
        Ok(this.0.borrow().acknowledged(member))
    }

    fn reached(&self) -> bool {
        self.0.borrow().reached()
    }

    fn count(&self) -> usize {
        self.0.borrow().count()
    }

    fn members(&self) -> usize {
        self.0.borrow().members()
    }

    fn required(&self) -> usize {
        self.0.borrow().required()
    }

    fn reset(this: Obj<Self>) -> Result<(), Error> {
        this.check_frozen()?;
        this.0.borrow_mut().reset();
        Ok(())
    }
}

fn intersects(members: Value, reads: Value, writes: Value) -> Result<bool, Error> {
    let members = integer(members, "members")?;
    let reads = integer(reads, "reads")?;
    let writes = integer(writes, "writes")?;
    Ok(quorum_machines::intersects(members, reads, writes))
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("QuorumMachines")?;
    module.define_singleton_method("intersects?", function!(intersects, 3))?;
    let class = module.define_class("Quorum", ruby.class_object())?;
    class.undef_default_alloc_func();
    class.const_set("CAPACITY", Core::CAPACITY)?;
    class.define_singleton_method("new", function!(Tracker::new, 2))?;
    class.define_singleton_method("majority", function!(Tracker::majority, 1))?;
    class.define_method("acknowledge", method!(Tracker::acknowledge, 1))?;
    class.define_method("acknowledged?", method!(Tracker::acknowledged, 1))?;
    class.define_method("reached?", method!(Tracker::reached, 0))?;
    class.define_method("count", method!(Tracker::count, 0))?;
    class.define_method("members", method!(Tracker::members, 0))?;
    class.define_method("required", method!(Tracker::required, 0))?;
    class.define_method("reset", method!(Tracker::reset, 0))
}

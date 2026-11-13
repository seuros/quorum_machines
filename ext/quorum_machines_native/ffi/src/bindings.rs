use magnus::typed_data::Obj;
use magnus::{Error, Integer, Ruby, Symbol, TryConvert, Value, function, method, prelude::*};
use quorum_machines::{Acknowledgement, Quorum, Refusal};
use std::cell::RefCell;

/// Seats compiled into the binding: the core's `N`.
const CAPACITY: usize = 64;

type Core = Quorum<CAPACITY>;

#[derive(Clone, Copy)]
struct Count(usize);

impl TryConvert for Count {
    fn try_convert(value: Value) -> Result<Self, Error> {
        if Integer::from_value(value).is_none() {
            return Err(Error::new(
                Ruby::get_with(value).exception_type_error(),
                "expected an Integer",
            ));
        }
        usize::try_convert(value).map(Self)
    }
}

const fn refusal(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::UnknownMember => "unknown_member",
    }
}

const fn acknowledgement(acknowledgement: Acknowledgement) -> &'static str {
    match acknowledgement {
        Acknowledgement::Counted => "counted",
        Acknowledgement::Duplicate => "duplicate",
    }
}

#[magnus::wrap(class = "QuorumMachines::Quorum", free_immediately, size)]
struct Tracker(RefCell<Core>);

impl Tracker {
    fn build(ruby: &Ruby, core: Option<Core>) -> Result<Self, Error> {
        core.map(|core| Self(RefCell::new(core))).ok_or_else(|| {
            Error::new(
                ruby.exception_range_error(),
                "require 1 <= required <= members <= 64",
            )
        })
    }

    fn new(ruby: &Ruby, members: Count, required: Count) -> Result<Self, Error> {
        Self::build(ruby, Core::new(members.0, required.0))
    }

    fn majority(ruby: &Ruby, members: Count) -> Result<Self, Error> {
        Self::build(ruby, Core::majority(members.0))
    }

    fn acknowledge(ruby: &Ruby, this: Obj<Self>, member: Count) -> Result<Symbol, Error> {
        this.check_frozen()?;
        let outcome = this.0.borrow_mut().acknowledge(member.0);
        Ok(ruby.to_symbol(outcome.map_or_else(refusal, acknowledgement)))
    }

    fn acknowledged(&self, member: Count) -> bool {
        self.0.borrow().acknowledged(member.0)
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

const fn intersects(members: Count, reads: Count, writes: Count) -> bool {
    quorum_machines::intersects(members.0, reads.0, writes.0)
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

use magnus::typed_data::Obj;
use magnus::{Error, Integer, Ruby, TryConvert, Value, function, method, prelude::*};
use quorum_machines::Quorum;
use std::cell::RefCell;

const MAX_MEMBERS: usize = 64;

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

#[magnus::wrap(class = "QuorumMachines::Quorum", free_immediately, size)]
struct Tracker {
    members: usize,
    core: RefCell<Quorum<MAX_MEMBERS>>,
}

impl Tracker {
    fn new(ruby: &Ruby, members: Count, required: Count) -> Result<Self, Error> {
        if members.0 == 0 || members.0 > MAX_MEMBERS || required.0 > members.0 {
            return Err(Error::new(
                ruby.exception_arg_error(),
                "require 1 <= required <= members <= 64",
            ));
        }
        let core = Quorum::new(required.0)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "threshold must be positive"))?;
        Ok(Self {
            members: members.0,
            core: RefCell::new(core),
        })
    }
    fn acknowledge(this: Obj<Self>, member: Count) -> Result<bool, Error> {
        this.check_frozen()?;
        Ok(member.0 < this.members && this.core.borrow_mut().acknowledge(member.0))
    }
    fn count(&self) -> usize {
        self.core.borrow().count()
    }
    fn reached(&self) -> bool {
        self.core.borrow().reached()
    }
    fn reset(this: Obj<Self>) -> Result<(), Error> {
        this.check_frozen()?;
        this.core.borrow_mut().reset();
        Ok(())
    }
}

const fn intersects(members: Count, reads: Count, writes: Count) -> bool {
    quorum_machines::intersects(members.0, reads.0, writes.0)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("QuorumMachines")?;
    module.const_set("MAX_MEMBERS", MAX_MEMBERS)?;
    module.define_singleton_method("_intersects", function!(intersects, 3))?;
    let class = module.define_class("Quorum", ruby.class_object())?;
    class.define_singleton_method("_new", function!(Tracker::new, 2))?;
    class.define_method("acknowledge", method!(Tracker::acknowledge, 1))?;
    class.define_method("count", method!(Tracker::count, 0))?;
    class.define_method("reached?", method!(Tracker::reached, 0))?;
    class.define_method("reset", method!(Tracker::reset, 0))
}

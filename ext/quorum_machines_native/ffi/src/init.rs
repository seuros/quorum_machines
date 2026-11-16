use magnus::{Error, Ruby, function, method, prelude::*};

use crate::{Core, Tracker, intersects};

/// Defines `QuorumMachines.intersects?` and `QuorumMachines::Quorum` when Ruby
/// loads the extension.
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

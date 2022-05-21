#![cfg_attr(not(feature = "stable"), feature(never_type))]

// Doctests (i don't have any??) don't compile with auto_import for some reason, so use explicit imports in this file
use std::panic::UnwindSafe;

#[cfg(test)]
mod tests;

/// Catastrophic evaluation: Evaluates an expression that always panics, catches the stack unwind, tries to downcast it to the given `T`, and returns that if successful, else it continues unwinding with the unknown value.
/// This macro allows for several expressions to be passed, and in that case it will evaluate to a tuple
#[macro_export]
macro_rules! catastrophe {
    ($($e:expr => $t:ty),* $(,)?) => {
        ($(
            match ::std::panic::catch_unwind(|| $e)
            .expect_err("Catastrophic success")
            .downcast::<$t>()
            {
                ::std::result::Result::Ok(boxed) => *boxed,
                ::std::result::Result::Err(err) => ::std::panic::resume_unwind(err),
            }
        ),*)
    };
}

/// Represents a catastrophic value ('static + Send)
///
/// Only types that are 'static + Send may be used for panicking, and Catastrophic is simply a clearer shorthand for that
pub trait Catastrophic: 'static + Send {}
impl<T: 'static + Send> Catastrophic for T {}

/// Represents a function that returns a catastrophic value by panicking and is unwind safe)
pub trait DisasterWaitingToHappen: UnwindSafe {
    // Unwinds the stack with an unknown type, but an expectation that the caller knows it and will catch it
    #[allow(non_snake_case)]
    fn HALT_AND_CATCH_FIRE(self) -> !;
}

#[cfg(not(feature = "stable"))]
type Never = !;
#[cfg(feature = "stable")]
enum Never {}

impl<T: FnOnce() -> Never + UnwindSafe> DisasterWaitingToHappen for T {
    fn HALT_AND_CATCH_FIRE(self) -> ! {
        self();
        // uninstantiable enum doesn't affect control flow, only the real ! type
        #[cfg(feature = "stable")]
        unreachable!()
    }
}

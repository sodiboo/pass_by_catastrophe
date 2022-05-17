#[cfg(test)]
mod tests;

use std::marker::Send;
use std::panic::UnwindSafe;

/// Catastrophic evaluation: Evaluates an expression that always panics, catches the stack unwind, tries to downcast it to the given `T`, and returns that if successful, else it continues unwinding with the unknown value.
/// This macro allows for several expressions to be passed, and in that case it will evaluate to a tuple
#[macro_export]
macro_rules! catastrophe {
    ($($e:expr => $t:ty),* $(,)?) => {
        ($(
            match std::panic::catch_unwind(|| $e)
            .expect_err("An instance of Never should be impossible. What the hell did you do?")
            .downcast::<$t>()
            {
                Ok(boxed) => *boxed,
                Err(err) => std::panic::resume_unwind(err),
            }
        ),*)
    };
}

/// Represents a catastrophic value ('static + Send)
///
/// Only types that are 'static + Send may be used for panicking, and Catastrophic is simply a clearer shorthand for that
pub trait Catastrophic: 'static + Send {}
impl<T: 'static + Send> Catastrophic for T {}

/// Represents a function that returns a catastrophic value (and is unwind safe)
pub trait DisasterWaitingToHappen: UnwindSafe {
    // Unwinds the stack with an unknown type, but an expectation that the caller knows it and will catch it
    #[allow(non_snake_case)]
    fn HALT_AND_CATCH_FIRE(self) -> !;
}

// implementation detail, so there's no reason to use a unified Never type from never crate. best to keep it dependency free
enum Never {}

impl<T: FnOnce() -> Never + UnwindSafe> DisasterWaitingToHappen for T {
    fn HALT_AND_CATCH_FIRE(self) -> ! {
        self();
        unreachable!()
    }
}

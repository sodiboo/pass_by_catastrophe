# [Pass by catastrophe](https://twitter.com/Dev14e/status/1526324763521871872)

This is a library that has two helper traits for conciceness, and a helper function for DRY code.

Values passed by catastrophe are returned via panicking, and unwind the stack. This library does not work if you abort on panic.

How does that work for parameters? Via ``FnOnce``, or rather, an equivalent to ``FnOnce`` that is usually an ``FnOnce`` but need not necessarily be ``FnOnce``: a ``DisasterWaitingToHappen`` is a type i created because rust requires opting into nightly features for the ``!`` type to be used anywhere other than function return types. As such, a ``DisasterWaitingToHappen`` is essentially an ``FnOnce() -> !``, just not using ``!`` in that currently-unstable position. Since ``!`` can be coerced into any type, such a function will also be an ``FnOnce() -> Never`` (``Never`` is a private implementation detail), and those implement ``DisasterWaitingToHappen``. Basically, parameters are passed such that instead of ``x``, you instead pass ``|| std::panic::panic_any(x)``.

By default, this package does require nightly toolchain for the ``never`` nightly feature, but the feature flag ``stable`` will make this library use an uninstantiable enum instead. I don't think there is literally any difference whatsoever, since that's not part of a public signature anywhere, and ``!`` is coerce to that private type anyways.

Because the unwind stack is typed as ``dyn Any``, when reading the return value of a catastrophic function, you must re-specify its type. Type inference obviously has no fucking clue why you are doing this, and refuses to do it's goddamn job.

-   Rust currently does not allow ``impl Trait`` and a normal generic parameter to the same function
-   ``Catastrophe`` is untyped, so for generic functions to work at all, you need them to be explicitly specified

As a result, you cannot use `impl DisasterWaitingToHappen` in a generic function. You have to pass an explicit `F: DisasterWaitingToHappen`

-   ``Disaster``s``WaitingToHappen`` are often unnamed closures, meaning you cannot put them in a generic parameter's position
-   Rust does not allow omitting parameters to infer them, and as a result you have to specify the catastrophe type
-   Rust *does* allow you to specify a generic type as _ to say "infer pls", and *that works*.

As a result, a generic catastrophic function almost always have to take equal amounts of `_` generic parameters as the amount of parameters.

The only exception to this is when assigning to a non-catastrophic value, i.e. a ``let`` binding, or something actually useful like that. In that case, you *can* likely omit the generics entirely and let rust figure it out by itself.

The ``catastrophe!`` macro has two uses:

1. If you call it with some expression that is the result of a catastrophic function call (i.e. ``!``) it will evaluate that expression and coerce it into the given type.
2. If you give it several ``Disaster``s``WaitingToHappen`` (ugh i hate it when type names get so long that the plural form is supposed to modify a word somewhere in the middle), you have to manually execute them, and provide their respective return types. The result is a tuple.

Both of these are the same implementation of the macro, so if you want to, you can use it to tuple multiple function calls. Really, a ``DisasterWaitingToHappen`` is just a trait with a catastrophic function, which happens to be called ``HALT_AND_CATCH_FIRE()``. Because they're the same impl with the same syntax, you have to call that function. A fitting name, since it does halt program execution and begins unwinding the stack.

The things in this library helps reduce this code:

```rust
use std::ops::Add;
use std::panic::UnwindSafe;
use std::panic::panic_any;
use never::Never;

fn add4<Num: Add<Num, Output = Num>, A, B, C, D>(a: A, b: B, c: C, d: D) -> !
where
    Num: 'static + Send,
    A: FnOnce() -> Never + UnwindSafe,
    B: FnOnce() -> Never + UnwindSafe,
    C: FnOnce() -> Never + UnwindSafe,
    D: FnOnce() -> Never + UnwindSafe,
{
    let a = *std::panic::catch_unwind(a).unwrap_err().downcast::<Num>().unwrap();
    let b = *std::panic::catch_unwind(b).unwrap_err().downcast::<Num>().unwrap();
    let c = *std::panic::catch_unwind(c).unwrap_err().downcast::<Num>().unwrap();
    let d = *std::panic::catch_unwind(d).unwrap_err().downcast::<Num>().unwrap();
    panic_any(a + b + c + d);
}
```

into just this:

```rust
use std::ops::Add;
use std::panic::panic_any;
use pass_by_catastrophe::Catastrophic;
use pass_by_catastrophe::DisasterWaitingToHappen;

fn add4<Num: Catastrophic, A, B, C, D>(a: A, b: B, c: C, d: D) -> !
where
    Num: Add<Num, Output = Num>,
    A: DisasterWaitingToHappen,
    B: DisasterWaitingToHappen,
    C: DisasterWaitingToHappen,
    D: DisasterWaitingToHappen,
{
    let (a, b, c, d) = catastrophe!(
        a.HALT_AND_CATCH_FIRE() => Num,
        b.HALT_AND_CATCH_FIRE() => Num,
        c.HALT_AND_CATCH_FIRE() => Num,
        d.HALT_AND_CATCH_FIRE() => Num,
    );
    panic_any(a + b + c + d);
}
```

Except that it's not really a fair comparison, since the code below using the ``catastrophe!`` macro will properly handle incorrectly-typed panics, and it will also properly handle the possibility of any of ``a``, ``b``, ``c``, ``d`` not panicking and actually returning, in case of unsafe fuckery or something. That example is one of the tests in [``tests.rs``](src/tests.rs), by the way

I do recommend the [magic-import](https://crates.io/crates/magic-import) crate, which allows the top of that example to be reduced further:

```rust
magic_import::magic!();

fn add4(...) { ... }
```

Oh, and you wanna know the **best part**? If you pass by catastrophe in your program, and somewhere you try to catch an ``i32``, but the code actually returns an ``i64``, then the error will propagate up all the way until it finds a catastrophic landing zone that **does** look for ``i64``. Have fun debugging that, sucker!
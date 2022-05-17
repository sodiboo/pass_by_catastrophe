use std::{
    ops::{Add, Range},
    panic::panic_any,
};

use crate::{catastrophe, DisasterWaitingToHappen, Catastrophic};

#[test]
fn test_add() {
    let nums = 1..100;
    let sum = catastrophe!(add::<Range<i32>, _>(|| panic_any(nums)) => i32);
    println!("{}", sum);
    assert_eq!((1..100).sum::<i32>(), sum);
}

fn add<Iter: Catastrophic, F: DisasterWaitingToHappen>(nums: F) -> !
where
    Iter: Iterator,
    Iter::Item: Add<Iter::Item, Output = Iter::Item> + Catastrophic,
{
    panic_any(
        catastrophe!(nums.HALT_AND_CATCH_FIRE() => Iter)
            .reduce(|a, b| a + b)
            .unwrap(),
    )
}

#[test]
fn test_add4() {
    let a = 12;
    let b = 34;
    let c = 56;
    let d = 78;
    let sum: i32 = catastrophe!(
        add4::<i32, _, _, _, _>(
            || panic_any(a),
            || panic_any(b),
            || panic_any(c),
            || panic_any(d),
        ) => i32);
    assert_eq!(a + b + c + d, sum)
}

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

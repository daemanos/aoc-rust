use std::cmp;
use num_traits::PrimInt;
use std::fmt::Debug;

pub fn gcd<T: PrimInt + Debug>(a: T, b: T) -> T {
    let zero = T::zero();

    let mut a = a;
    let mut b = b;
    while b > zero {
        let m = a % b;
        (a, b) = (b, m);
    }

    a
}

pub fn lcm<T: PrimInt + Debug>(a: T, b: T) -> T {
    let (x, y) = (cmp::min(a, b), cmp::max(a, b));
    let gcd = gcd(a, b);
    x * (y / gcd)
}

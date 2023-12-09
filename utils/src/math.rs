use std::cmp;
use num_traits::PrimInt;

pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    let zero = T::zero();

    let mut r = (a, b);
    while r.1 != zero {
        let m = r.0 % r.1;
        r = (r.1, m);
    }

    r.0
}

pub fn ext_gcd<T: PrimInt>(a: T, b: T) -> (T, T, T) {
    let zero = T::zero();
    let one = T::one();

    let mut r = (a, b);
    let mut s = (one, zero);
    let mut t = (zero, one);

    while r.1 != zero {
        let q = r.0 / r.1;

        let old = r;
        r = (old.1, old.0 - q * old.1);

        let old = s;
        s = (old.1, old.0 - q * old.1);

        let old = t;
        t = (old.1, old.0 - q * old.1);
    }

    (r.0, s.0, t.0)
}

pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
    let (x, y) = (cmp::min(a, b), cmp::max(a, b));
    let gcd = gcd(a, b);
    x * (y / gcd)
}

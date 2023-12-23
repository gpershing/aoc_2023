pub trait Argm {
    // argmax[n] b * n < a
    fn argmax_lt(a: Self, b: Self) -> Self;
    // argmax[n] b * n <= a
    fn argmax_le(a: Self, b: Self) -> Self;
    // argmin[n] b * n > a
    fn argmin_gt(a: Self, b: Self) -> Self;
    // argmin[n] b * n >= a
    fn argmin_ge(a: Self, b: Self) -> Self;
}

impl Argm for u32 {
    fn argmax_lt(a: Self, b: Self) -> Self { a / b }
    fn argmax_le(a: Self, b: Self) -> Self { (a - 1) / b }
    fn argmin_gt(a: Self, b: Self) -> Self { (a / b) + 1 }
    fn argmin_ge(a: Self, b: Self) -> Self { (a + (b - 1)) / b }
}
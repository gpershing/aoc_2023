use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T, S> Add<Vector2<S>> for Vector2<T> where T: Add<S> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<S>) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a, T, S> Add<Vector2<S>> for &'a Vector2<T> where &'a T: Add<S> {
    type Output = Vector2<<&'a T as Add<S>>::Output>;

    fn add(self, rhs: Vector2<S>) -> Self::Output {
        Self::Output { x: &self.x + rhs.x, y: &self.y + rhs.y }
    }
}

impl<T, S> AddAssign<Vector2<S>> for Vector2<T> where T: AddAssign<S> {
    fn add_assign(&mut self, rhs: Vector2<S>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, S> Sub<Vector2<S>> for Vector2<T> where T: Sub<S> {
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Vector2<S>) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<'a, T, S> Sub<Vector2<S>> for &'a Vector2<T> where &'a T: Sub<S> {
    type Output = Vector2<<&'a T as Sub<S>>::Output>;

    fn sub(self, rhs: Vector2<S>) -> Self::Output {
        Self::Output { x: &self.x - rhs.x, y: &self.y - rhs.y }
    }
}

impl<T, S> SubAssign<Vector2<S>> for Vector2<T> where T: SubAssign<S> {
    fn sub_assign(&mut self, rhs: Vector2<S>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T, S> Mul<S> for Vector2<T> where T: Mul<S>, S: Copy {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: S) -> Self::Output {
        Self::Output { x: self.x * rhs, y : self.y * rhs }
    }
}

impl<T, S> MulAssign<S> for Vector2<T> where T: MulAssign<S>, S: Copy {
    fn mul_assign(&mut self, rhs: S) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T, S> Div<S> for Vector2<T> where T: Div<S>, S: Copy {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: S) -> Self::Output {
        Self::Output { x: self.x / rhs, y : self.y / rhs }
    }
}

impl<T, S> DivAssign<S> for Vector2<T> where T: DivAssign<S>, S: Copy {
    fn div_assign(&mut self, rhs: S) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl From<Vector2<f32>> for Vector2<f64> { fn from(value: Vector2<f32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<i8>> for Vector2<f32> { fn from(value: Vector2<i8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i8>> for Vector2<f64> { fn from(value: Vector2<i8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i8>> for Vector2<i16> { fn from(value: Vector2<i8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i8>> for Vector2<i32> { fn from(value: Vector2<i8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i8>> for Vector2<i64> { fn from(value: Vector2<i8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i8>> for Vector2<i128> { fn from(value: Vector2<i8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i8>> for Vector2<isize> { fn from(value: Vector2<i8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<i16>> for Vector2<f32> { fn from(value: Vector2<i16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i16>> for Vector2<f64> { fn from(value: Vector2<i16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i16>> for Vector2<i32> { fn from(value: Vector2<i16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i16>> for Vector2<i64> { fn from(value: Vector2<i16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i16>> for Vector2<i128> { fn from(value: Vector2<i16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i16>> for Vector2<isize> { fn from(value: Vector2<i16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<i32>> for Vector2<f64> { fn from(value: Vector2<i32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i32>> for Vector2<i64> { fn from(value: Vector2<i32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<i32>> for Vector2<i128> { fn from(value: Vector2<i32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<i64>> for Vector2<i128> { fn from(value: Vector2<i64>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<u8>> for Vector2<f32> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<f64> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<i16> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<i32> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<i64> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<i128> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<isize> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<u16> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<u32> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<u64> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u8>> for Vector2<usize> { fn from(value: Vector2<u8>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<u16>> for Vector2<f32> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u16>> for Vector2<f64> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u16>> for Vector2<i32> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u16>> for Vector2<i64> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u16>> for Vector2<i128> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u16>> for Vector2<u32> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u16>> for Vector2<u64> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u16>> for Vector2<usize> { fn from(value: Vector2<u16>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<u32>> for Vector2<f64> { fn from(value: Vector2<u32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u32>> for Vector2<i64> { fn from(value: Vector2<u32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u32>> for Vector2<i128> { fn from(value: Vector2<u32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }
impl From<Vector2<u32>> for Vector2<u64> { fn from(value: Vector2<u32>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl From<Vector2<u64>> for Vector2<i128> { fn from(value: Vector2<u64>) -> Self { Self { x: value.x.into(), y: value.y.into() } } }

impl TryFrom<Vector2<i8>> for Vector2<u8> { type Error = <u8 as TryFrom<i8>>::Error; fn try_from(value: Vector2<i8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i8>> for Vector2<u16> { type Error = <u16 as TryFrom<i8>>::Error; fn try_from(value: Vector2<i8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i8>> for Vector2<u32> { type Error = <u32 as TryFrom<i8>>::Error; fn try_from(value: Vector2<i8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i8>> for Vector2<u64> { type Error = <u64 as TryFrom<i8>>::Error; fn try_from(value: Vector2<i8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i8>> for Vector2<u128> { type Error = <u128 as TryFrom<i8>>::Error; fn try_from(value: Vector2<i8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i8>> for Vector2<usize> { type Error = <usize as TryFrom<i8>>::Error; fn try_from(value: Vector2<i8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<i16>> for Vector2<i8> { type Error = <i8 as TryFrom<i16>>::Error; fn try_from(value: Vector2<i16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i16>> for Vector2<u8> { type Error = <u8 as TryFrom<i16>>::Error; fn try_from(value: Vector2<i16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i16>> for Vector2<u16> { type Error = <u16 as TryFrom<i16>>::Error; fn try_from(value: Vector2<i16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i16>> for Vector2<u32> { type Error = <u32 as TryFrom<i16>>::Error; fn try_from(value: Vector2<i16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i16>> for Vector2<u64> { type Error = <u64 as TryFrom<i16>>::Error; fn try_from(value: Vector2<i16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i16>> for Vector2<u128> { type Error = <u128 as TryFrom<i16>>::Error; fn try_from(value: Vector2<i16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i16>> for Vector2<usize> { type Error = <usize as TryFrom<i16>>::Error; fn try_from(value: Vector2<i16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<i32>> for Vector2<i8> { type Error = <i8 as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<i16> { type Error = <i16 as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<isize> { type Error = <isize as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<u8> { type Error = <u8 as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<u16> { type Error = <u16 as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<u32> { type Error = <u32 as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<u64> { type Error = <u64 as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<u128> { type Error = <u128 as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i32>> for Vector2<usize> { type Error = <usize as TryFrom<i32>>::Error; fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<i64>> for Vector2<i8> { type Error = <i8 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<i16> { type Error = <i16 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<i32> { type Error = <i32 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<isize> { type Error = <isize as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<u8> { type Error = <u8 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<u16> { type Error = <u16 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<u32> { type Error = <u32 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<u64> { type Error = <u64 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<u128> { type Error = <u128 as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i64>> for Vector2<usize> { type Error = <usize as TryFrom<i64>>::Error; fn try_from(value: Vector2<i64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<i128>> for Vector2<i8> { type Error = <i8 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<i16> { type Error = <i16 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<i32> { type Error = <i32 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<i64> { type Error = <i64 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<isize> { type Error = <isize as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<u8> { type Error = <u8 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<u16> { type Error = <u16 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<u32> { type Error = <u32 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<u64> { type Error = <u64 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<u128> { type Error = <u128 as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<i128>> for Vector2<usize> { type Error = <usize as TryFrom<i128>>::Error; fn try_from(value: Vector2<i128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<isize>> for Vector2<i8> { type Error = <i8 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<i16> { type Error = <i16 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<i32> { type Error = <i32 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<i64> { type Error = <i64 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<i128> { type Error = <i128 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<u8> { type Error = <u8 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<u16> { type Error = <u16 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<u32> { type Error = <u32 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<u64> { type Error = <u64 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<u128> { type Error = <u128 as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<isize>> for Vector2<usize> { type Error = <usize as TryFrom<isize>>::Error; fn try_from(value: Vector2<isize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<u8>> for Vector2<i8> { type Error = <i8 as TryFrom<u8>>::Error; fn try_from(value: Vector2<u8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u8>> for Vector2<u128> { type Error = <u128 as TryFrom<u8>>::Error; fn try_from(value: Vector2<u8>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<u16>> for Vector2<i8> { type Error = <i8 as TryFrom<u16>>::Error; fn try_from(value: Vector2<u16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u16>> for Vector2<i16> { type Error = <i16 as TryFrom<u16>>::Error; fn try_from(value: Vector2<u16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u16>> for Vector2<isize> { type Error = <isize as TryFrom<u16>>::Error; fn try_from(value: Vector2<u16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u16>> for Vector2<u8> { type Error = <u8 as TryFrom<u16>>::Error; fn try_from(value: Vector2<u16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u16>> for Vector2<u128> { type Error = <u128 as TryFrom<u16>>::Error; fn try_from(value: Vector2<u16>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<u32>> for Vector2<i8> { type Error = <i8 as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u32>> for Vector2<i16> { type Error = <i16 as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u32>> for Vector2<i32> { type Error = <i32 as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u32>> for Vector2<isize> { type Error = <isize as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u32>> for Vector2<u8> { type Error = <u8 as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u32>> for Vector2<u16> { type Error = <u16 as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u32>> for Vector2<u128> { type Error = <u128 as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u32>> for Vector2<usize> { type Error = <usize as TryFrom<u32>>::Error; fn try_from(value: Vector2<u32>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<u64>> for Vector2<i8> { type Error = <i8 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<i16> { type Error = <i16 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<i32> { type Error = <i32 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<i64> { type Error = <i64 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<isize> { type Error = <isize as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<u8> { type Error = <u8 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<u16> { type Error = <u16 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<u32> { type Error = <u32 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<u128> { type Error = <u128 as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u64>> for Vector2<usize> { type Error = <usize as TryFrom<u64>>::Error; fn try_from(value: Vector2<u64>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<u128>> for Vector2<i8> { type Error = <i8 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<i16> { type Error = <i16 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<i32> { type Error = <i32 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<i64> { type Error = <i64 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<i128> { type Error = <i128 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<isize> { type Error = <isize as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<u8> { type Error = <u8 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<u16> { type Error = <u16 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<u32> { type Error = <u32 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<u64> { type Error = <u64 as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<u128>> for Vector2<usize> { type Error = <usize as TryFrom<u128>>::Error; fn try_from(value: Vector2<u128>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }

impl TryFrom<Vector2<usize>> for Vector2<i8> { type Error = <i8 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<i16> { type Error = <i16 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<i32> { type Error = <i32 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<i64> { type Error = <i64 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<i128> { type Error = <i128 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<isize> { type Error = <isize as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<u8> { type Error = <u8 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<u16> { type Error = <u16 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<u32> { type Error = <u32 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<u64> { type Error = <u64 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
impl TryFrom<Vector2<usize>> for Vector2<u128> { type Error = <u128 as TryFrom<usize>>::Error; fn try_from(value: Vector2<usize>) -> Result<Self, Self::Error> { Ok(Self { x: value.x.try_into()?, y: value.y.try_into()? }) } }
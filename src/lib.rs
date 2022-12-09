#![feature(array_zip)]
pub use proc::main;
use std::ops::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Vector<T, const N: usize>(pub [T; N]);
pub type VU<const N: usize> = Vector<usize, N>;
pub type VI<const N: usize> = Vector<isize, N>;

#[macro_export]
macro_rules! v {
    ($elem: expr; $n:expr) => ($crate::Vector([$elem; $n]));
    ($($x:expr),+ $(,)?) => ($crate::Vector([$($x),+]));
}

impl<const N: usize> VI<N> {
    pub fn unit(&self) -> Self {
        Vector(self.0.map(|n| n.signum()))
    }

    pub fn abs(&self) -> Self {
        Vector(self.0.map(|n| n.abs()))
    }
}

impl<T: Mul<Output = T> + Copy, const N: usize> Vector<T, N> {
    pub fn prod(&self, other: Self) -> Self {
        Vector(self.0.zip(other.0).map(|(a, b)| a * b))
    }
}

impl<T: Add<Output = T> + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: Self) -> Self::Output {
        Vector(self.0.zip(other.0).map(|(a, b)| a + b))
    }
}

impl<T: Sub<Output = T> + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, other: Self) -> Self::Output {
        Vector(self.0.zip(other.0).map(|(a, b)| a - b))
    }
}

impl<T: AddAssign + Copy, const N: usize> AddAssign<Self> for Vector<T, N> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self.0[i] += other.0[i];
        }
    }
}

impl<T: SubAssign + Copy, const N: usize> SubAssign<Self> for Vector<T, N> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..N {
            self.0[i] -= other.0[i];
        }
    }
}

impl<T: Mul<Output = T> + Copy, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vector(self.0.map(|n| n * rhs))
    }
}

impl<T: MulAssign + Copy, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.0[i] *= rhs;
        }
    }
}

use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct TotalF64(pub f64);

impl PartialEq for TotalF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for TotalF64 {}

impl Hash for TotalF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialOrd for TotalF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.total_cmp(&other.0))
    }
}

impl Ord for TotalF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Add for TotalF64 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        TotalF64(self.0 + other.0)
    }
}

impl Sub for TotalF64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        TotalF64(self.0 - other.0)
    }
}

impl Mul for TotalF64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        TotalF64(self.0 * other.0)
    }
}

impl Div for TotalF64 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        TotalF64(self.0 / other.0)
    }
}

impl Add<&TotalF64> for &TotalF64 {
    type Output = TotalF64;

    fn add(self, other: &TotalF64) -> Self::Output {
        TotalF64(self.0 + other.0)
    }
}

impl Sub<&TotalF64> for &TotalF64 {
    type Output = TotalF64;

    fn sub(self, other: &TotalF64) -> Self::Output {
        TotalF64(self.0 - other.0)
    }
}

impl Mul<&TotalF64> for &TotalF64 {
    type Output = TotalF64;

    fn mul(self, other: &TotalF64) -> Self::Output {
        TotalF64(self.0 * other.0)
    }
}

impl Div<&TotalF64> for &TotalF64 {
    type Output = TotalF64;

    fn div(self, other: &TotalF64) -> Self::Output {
        TotalF64(self.0 / other.0)
    }
}

impl fmt::Display for TotalF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f64> for TotalF64 {
    fn from(value: f64) -> Self {
        TotalF64(value)
    }
}

impl From<TotalF64> for f64 {
    fn from(value: TotalF64) -> Self {
        value.0
    }
}

/// Assumes has canonical elements of being signed.
pub trait Signed {
    fn signum(&self) -> Self;
}

/// Assumes can be zero or negative, that why reasonably ask for a positive check.
pub trait IsPositive {
    fn is_positive(&self) -> bool;
}

pub trait SaturatingSub<Rhs: Sized + Copy>: Sized + Copy {
    type Output;
    /// Saturating subtraction. Computes `self - other`, saturating at the relevant high or low boundary of
    /// the type.
    fn saturating_sub(&self, v: Rhs) -> Self::Output;
}

/// `Tie::Break(Integer::ToZero)`
pub trait CheckedDiv<Rhs> {
    type Output;
    fn checked_div(&self, v: Rhs) -> Option<Self::Output>;
}

/// `Tie::Break(Integer::Up)`
/// num_traits force Div impl with lhs, rhs and output be same -
/// does not allows proper typing.
/// so we have own.
pub trait DivCeil<Rhs> {
    type Output;
    fn div_ceil(&self, v: Rhs) -> Self::Output;
}

pub trait CheckedMul<Rhs> {
    type Output;
    fn checked_mul(&self, v: Rhs) -> Option<Self::Output>;
}

pub trait Next {
    type Output;
    fn next() -> Self::Output;
}

/// `Tie::Break(Integer::ToZero)`
pub trait SaturatingDiv<Rhs: Sized + Copy>: Sized + Copy {
    type Output;
    fn saturating_div(&self, v: Rhs) -> Self::Output;
}

pub trait CheckedAdd<Rhs: Sized + Copy>: Sized + Copy {
    type Output;
    fn checked_add(&self, v: Rhs) -> Option<Self::Output>;
}

pub trait CheckedSub<Rhs: Sized + Copy>: Sized + Copy {
    type Output;
    fn checked_sub(&self, v: Rhs) -> Option<Self::Output>;
}

use crate::traits::*;
use std::num::*;

macro_rules! impl_signed {
    ($($t:ty)*) => {
        $(
            impl Signed for $t {
                #[inline]
                fn signum(&self) -> Self {
                    (*self).signum()
                }
            }
        )*
    }
}

impl_signed!(i8 i16 i32 i64 i128 isize);

macro_rules! impl_signed_nonzero {
    ($($t:ident)*) => {
        $(
            impl Signed for $t {
                #[inline]
                fn signum(&self) -> Self {
                    if self.get() > 0 {
                        $t::new(1).unwrap()
                    } else {
                        $t::new(-1).unwrap()
                    }
                }
            }
        )*
    }
}
impl_signed_nonzero!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);

macro_rules! impl_is_positive {
    ($($t:ty)*) => {
        $(
            impl IsPositive for $t {
                #[inline]
                fn is_positive(&self) -> bool {
                    (*self).is_positive()
                }
            }
        )*
    }
}
impl_is_positive!(i8 i16 i32 i64 i128 isize);
impl_is_positive!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);

macro_rules! impl_is_positive_unsigned {
    ($($t:ty)*) => {
        $(
            impl IsPositive for $t {
                #[inline]
                fn is_positive(&self) -> bool {
                    *self > 0
                }
            }
        )*
    }
}
impl_is_positive_unsigned!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_is_positive_nonzero_unsigned {
    ($($t:ty)*) => {
        $(
            impl IsPositive for $t {
                #[inline]
                fn is_positive(&self) -> bool {
                    true
                }
            }
        )*
    }
}
impl_is_positive_nonzero_unsigned!(NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);

macro_rules! impl_math_traits {
    ($($t:ty)*) => {
        $(
            impl SaturatingSub<$t> for $t {
                type Output = $t;
                #[inline]
                fn saturating_sub(&self, v: $t) -> Self::Output {
                    (*self).saturating_sub(v)
                }
            }

            impl CheckedDiv<$t> for $t {
                type Output = $t;
                #[inline]
                fn checked_div(&self, v: $t) -> Option<Self::Output> {
                    (*self).checked_div(v)
                }
            }

            impl DivCeil<$t> for $t {
                type Output = $t;
                #[inline]
                fn div_ceil(&self, v: $t) -> Self::Output {
                    let d = *self / v;
                    let r = *self % v;
                    if (r > 0 && v > 0) || (r < 0 && v < 0) {
                        d + 1
                    } else {
                        d
                    }
                }
            }

            impl CheckedMul<$t> for $t {
                type Output = $t;
                #[inline]
                fn checked_mul(&self, v: $t) -> Option<Self::Output> {
                    (*self).checked_mul(v)
                }
            }

            impl SaturatingDiv<$t> for $t {
                type Output = $t;
                #[inline]
                fn saturating_div(&self, v: $t) -> Self::Output {
                    (*self).saturating_div(v)
                }
            }

            impl CheckedAdd<$t> for $t {
                type Output = $t;
                #[inline]
                fn checked_add(&self, v: $t) -> Option<Self::Output> {
                    (*self).checked_add(v)
                }
            }

            impl CheckedSub<$t> for $t {
                type Output = $t;
                #[inline]
                fn checked_sub(&self, v: $t) -> Option<Self::Output> {
                    (*self).checked_sub(v)
                }
            }
        )*
    }
}

impl_math_traits!(i8 i16 i32 i64 i128 isize);
impl_math_traits!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_math_traits_nonzero {
    ($($t:ident, $inner:ty)*) => {
        $(
            impl CheckedDiv<$t> for $t {
                type Output = $t;
                #[inline]
                fn checked_div(&self, v: $t) -> Option<Self::Output> {
                    let res = self.get().checked_div(v.get())?;
                    $t::new(res)
                }
            }

            impl DivCeil<$t> for $t {
                type Output = $t;
                #[inline]
                fn div_ceil(&self, v: $t) -> Self::Output {
                    let d = self.get() / v.get();
                    let r = self.get() % v.get();
                    let res = if (r > 0 && v.get() > 0) || (r < 0 && v.get() < 0) {
                        d + 1
                    } else {
                        d
                    };
                    $t::new(res).expect("DivCeil resulted in zero")
                }
            }

            impl CheckedMul<$t> for $t {
                type Output = $t;
                #[inline]
                fn checked_mul(&self, v: $t) -> Option<Self::Output> {
                    self.get().checked_mul(v.get()).and_then($t::new)
                }
            }
            
            impl SaturatingDiv<$t> for $t {
                type Output = $t;
                #[inline]
                fn saturating_div(&self, v: $t) -> Self::Output {
                    let res = self.get().saturating_div(v.get());
                    $t::new(res).expect("SaturatingDiv resulted in zero")
                }
            }
        )*
    }
}

impl_math_traits_nonzero!(
    NonZeroI8, i8
    NonZeroI16, i16
    NonZeroI32, i32
    NonZeroI64, i64
    NonZeroI128, i128
    NonZeroIsize, isize
    NonZeroU8, u8
    NonZeroU16, u16
    NonZeroU32, u32
    NonZeroU64, u64
    NonZeroU128, u128
    NonZeroUsize, usize
);

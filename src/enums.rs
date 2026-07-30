//! Rust `-5 / 3` is just `Tie::Break(Integer::Up)`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integer {
    /// Up, Ceil, positive infinity.
    /// 2.5, 2.3, 2.7 -> 3
    /// -2.5, -2.3, -2.7 -> -2
    ///
    /// `Break`
    /// Round to the nearest integer,
    /// 2.3 -> 2,  2.5 -> 3, 2.7 -> 3
    /// -2.3 -> -2, -2.5 -> -2, -2.7 -> -3    
    Up,
    /// Down, Floor. negative infinity.
    /// 2.5, 2.3, 2.7 -> 2
    /// -2.5, -2.3, -2.7 -> -3
    /// `Break`:
    /// Round to the nearest integer,
    /// 2.3 -> 2,  2.5 -> 2, 2.7 -> 3
    /// -2.3 -> -2, -2.5 -> -3, -2.7 -> -3
    Down,
    /// ToZero, Trunc.
    /// 2.5, 2.3, 2.7 -> 2
    /// -2.5, -2.3, -2.7 -> -2
    ///
    /// `Break`:
    /// Round to the nearest integer,
    /// 2.3 -> 2,  2.5 -> 2, 2.7 -> 3
    /// -2.3 -> -2, -2.5 -> -2, -2.7 -> -3
    ToZero,
    /// AwayFromZero, Round, Infinity
    /// 2.5, 2.3, 2.7 -> 3
    /// -2.5, -2.3, -2.7 -> -3
    ///
    /// `Break`:
    /// Round to the nearest integer,
    /// 2.3 -> 2,  2.5 -> 3, 2.7 -> 3
    /// -2.3 -> -2, -2.5 -> -3, -2.7 -> -3
    AwayFromZero,
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::Integer {}
    impl Sealed for super::OneSidedInteger {}
}

pub trait TieBehavior: private::Sealed {}
impl TieBehavior for Integer {}
impl TieBehavior for OneSidedInteger {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tie<T: TieBehavior> {
    Ignore(T),
    Break(T),
    /// Banker's Rounding
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OneSidedInteger {
    /// Pushes the number higher.
    /// Acts as `Up`, `Ceil`, and `AwayFromZero`.
    Higher,

    /// Pulls the number lower.
    /// Acts as `Down`, `Floor`, `Truncate`, and `ToZero`.
    Lower,
}

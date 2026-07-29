enum Integer {
    /// Up, Ceil.
    /// 2.5, 2.3, 2.7 -> 3
    /// -2.5, -2.3, -2.7 -> -2
    /// 
    /// `Break`
   /// Round to the nearest integer, 
    /// 2.3 -> 2,  2.5 -> 3, 2.7 -> 3
    /// -2.3 -> -2, -2.5 -> -2, -2.7 -> -3    
    Up,
    /// Down, Floor.
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

enum Tie {
    Ignore,
    Break,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Rounding {
    NearInfinity,
    PassMinMax,
}

pub enum RoundingMode {
    Ceil,
    Floor,
    Expand,
    Trunc,
    HalfCeil,
    HalfFloor,
    #[default]
    HalfExpand,
    HalfTrunc,
    HalfEven,
}


pub enum RoundDirection {
    /// Floor the value, ie. 1.9 => 1, 1.1 => 1, 1.5 => 1
    Floor,
    /// Ceiling the value, ie. 1.9 => 2, 1.1 => 2, 1.5 => 2
    Ceiling,
}


/// An enum that specifies how a value should be rounded.
///
/// A `RoundingMode` can often be specified when a function conceptually returns a value of one
/// type, but must be rounded to another type. The most common case is a conceptually real-valued
/// function whose result must be rounded to an integer, like
/// [`div_round`](crate::num::arithmetic::traits::DivRound::div_round).
///
/// # Examples
/// Here are some examples of how floating-point values would be rounded to integer values using the
/// different `RoundingMode`s.
///
/// | x    | `Floor` | `Ceiling` | `Down` | `Up` | `Nearest` | `Exact`    |
/// |------|---------|-----------|--------|------|-----------|------------|
/// |  3.0 |       3 |         3 |      3 |    3 |         3 |          3 |
/// |  3.2 |       3 |         4 |      3 |    4 |         3 | `panic!()` |
/// |  3.8 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  3.5 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  4.5 |       4 |         5 |      4 |    5 |         4 | `panic!()` |
/// | -3.2 |      -4 |        -3 |     -3 |   -4 |        -3 | `panic!()` |
/// | -3.8 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -3.5 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -4.5 |      -5 |        -4 |     -4 |   -5 |        -4 | `panic!()` |
///
/// Sometimes a `RoundingMode` is used in an unusual context, such as rounding an integer to a
/// floating-point number, in which case further explanation of its behavior is provided at the
/// usage site.
///
/// A `RoundingMode` takes up 1 byte of space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RoundingMode {
    /// Applies the function $x \mapsto \operatorname{sgn}(x) \lfloor |x| \rfloor$. In other words,
    /// the value is rounded towards 0.
    Down,
    /// Applies the function $x \mapsto \operatorname{sgn}(x) \lceil |x| \rceil$. In other words,
    /// the value is rounded away from 0.
    Up,
    /// Applies the floor function: $x \mapsto \lfloor x \rfloor$. In other words, the value is
    /// rounded towards $-\infty$.
    Floor,
    /// Applies the ceiling function: $x \mapsto \lceil x \rceil$. In other words, the value is
    /// rounded towards $\infty$.
    Ceiling,
    /// Applies the function
    /// $$
    ///   x \mapsto \\begin{cases}
    ///       \lfloor x \rfloor & x - \lfloor x \rfloor < \frac{1}{2} \\\\
    ///       \lceil x \rceil & x - \lfloor x \rfloor > \frac{1}{2} \\\\
    ///       \lfloor x \rfloor &
    ///  x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and}
    ///         \\ \lfloor x \rfloor \\ \text{is even} \\\\
    ///       \lceil x \rceil &
    ///  x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor x \rfloor \\ \text{is odd.}
    ///   \\end{cases}
    /// $$
    /// In other words, it rounds to the nearest integer, and when there's a tie, it rounds to the
    /// nearest even integer. This is also called _bankers' rounding_ and is often used as a
    /// default.
    Nearest,
    /// Panics if the value is not already rounded.
    Exact,
}

/// Rounding modes.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum RoundingMode {
    /// Skip rounding operation.
    None = 1,

    /// Round half toward positive infinity.
    Up = 2,

    /// Round half toward negative infinity.
    Down = 4,

    /// Round half toward zero.
    ToZero = 8,

    /// Round half away from zero.
    FromZero = 16,

    /// Round half to even.
    ToEven = 32,

    /// Round half to odd.
    ToOdd = 64,
}


/// Rounding Mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundingMode {
    RoundToNearestEven = 0b000,
    RoundTowardsZero = 0b001,
    RoundDown = 0b010,
    RoundUp = 0b011,
    RoundToNearestMaxMagnitude = 0b100,
    Invalid = 0b111,
}


pub enum RoundMode {
    #[default]
    HalfToEven,
    HalfAwayFromZero,
    ToZero,
}

//! # Round numbers and durations to a given factor
//!
//! This provides an implementation of rounding for various values, including
//! the the native number types and [`core::time::Duration`] (also known as
//! `std::time::Duration`).
//!
//! The [`Roundable`] trait adds the following functions to roundable values:
//!
//!  * [`Roundable::try_round_to(factor,
//!    tie_strategy)`](Roundable::try_round_to()) (returns `None` on overflow)
//!  * [`Roundable::round_to(factor, tie_strategy)`](Roundable::round_to())
//!    (panics on overflow)
//!
//! ### Example
//!
//! ```rust
//! use roundable::{Roundable, Tie};
//!
//! assert!(310 == 314.round_to(10, Tie::Up));
//! assert!(300.0 == 314.1.round_to(100.0, Tie::Up));
//!
//! // To avoid panicking on overflow:
//! assert!(Some(260) == 255.try_round_to(10, Tie::Up));
//! assert!(None == 255u8.try_round_to(10, Tie::Up));
//! ```
//!
//! ## Tie strategies
//!
//! “Ties” are numbers exactly halfway between two round numbers, e.g. 0.5 when
//! rounding to the nearest whole number. Traditionally, ties are resolved by
//! picking the higher number, but there are other strategies. `Roundable`
//! supports the following rules:
//!
//!   * [`Tie::Up`]: Round ties up (what most people consider correct).
//!   * [`Tie::Down`]: Round ties down.
//!   * [`Tie::TowardZero`]: Round ties toward zero.
//!   * [`Tie::AwayFromZero`]: Round ties away from zero.
//!   * [`Tie::TowardEven`]: Round ties toward the “even” number (see docs).
//!   * [`Tie::TowardOdd`]: Round ties toward the “odd” number (see docs).
//!
//! ## Rounding `Duration`
//!
//! [`Duration`](core::time::Duration) can be rounded to a `Duration` factor,
//! just like a number type. For convenience, there are a number of
//! [constants](#constants) that can be used to make rounding `Duration` easier.
//!
//! ```rust
//! use roundable::{SECOND, MINUTE, Roundable, Tie};
//! use std::time::Duration;
//!
//! assert!(Duration::ZERO == Duration::from_millis(314).round_to(SECOND, Tie::Up));
//! assert!(MINUTE == Duration::from_millis(59_500).round_to(SECOND, Tie::Up));
//! ```
//!
//! ## `#![no_std]` by default
//!
//! You can use this crate with or without `std` and `alloc`. You do not need to
//! enable or disable features either way.
//!
//! ## Minimum supported Rust version
//!
//! Currently the minimum supported Rust version (MSRV) is **1.56.1**. Future
//! increases in the MSRV will require a major version bump.

// Lint configuration in Cargo.toml isn’t supported by cargo-geiger.
#![forbid(unsafe_code)]
#![no_std]

mod duration;
pub use duration::*;
mod float;
mod int;

/// How to handle a value that is exactly half, e.g. `5.round_to(10, ...)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tie {
    /// Round half up (what most people consider correct).
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(10 == 5.round_to(10, Tie::Up));
    /// assert!(0 == (-5).round_to(10, Tie::Up));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::Up));
    /// assert!(10 == 6.round_to(10, Tie::Up));
    /// assert!(0 == (-4).round_to(10, Tie::Up));
    /// assert!(-10 == (-6).round_to(10, Tie::Up));
    /// ```
    Up,

    /// Round half down.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(0 == 5.round_to(10, Tie::Down));
    /// assert!(-10 == (-5).round_to(10, Tie::Down));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::Down));
    /// assert!(10 == 6.round_to(10, Tie::Down));
    /// assert!(0 == (-4).round_to(10, Tie::Down));
    /// assert!(-10 == (-6).round_to(10, Tie::Down));
    /// ```
    Down,

    /// Round half toward zero.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(0 == 5.round_to(10, Tie::TowardZero));
    /// assert!(0 == (-5).round_to(10, Tie::TowardZero));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::TowardZero));
    /// assert!(10 == 6.round_to(10, Tie::TowardZero));
    /// assert!(0 == (-4).round_to(10, Tie::TowardZero));
    /// assert!(-10 == (-6).round_to(10, Tie::TowardZero));
    /// ```
    TowardZero,

    /// Round half away from zero.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(10 == 5.round_to(10, Tie::AwayFromZero));
    /// assert!(-10 == (-5).round_to(10, Tie::AwayFromZero));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::AwayFromZero));
    /// assert!(10 == 6.round_to(10, Tie::AwayFromZero));
    /// assert!(0 == (-4).round_to(10, Tie::AwayFromZero));
    /// assert!(-10 == (-6).round_to(10, Tie::AwayFromZero));
    /// ```
    AwayFromZero,

    /// Round half toward even.
    ///
    /// “Even” has a special meaning here since we only care about round
    /// values. If we are rounding to the nearest 10, then 0 is even, 10 is
    /// odd, 20 is even, and so on.
    ///
    /// If we are rounding to whole numbers, then even and odd have the
    /// conventional meaning.
    ///
    /// In general, a multiple of factor _n_ is even if `(n / factor) % 2 ==
    /// 0`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(20 == 15.round_to(10, Tie::TowardEven));
    /// assert!(0 == 5.round_to(10, Tie::TowardEven));
    /// assert!(0 == (-5).round_to(10, Tie::TowardEven));
    /// assert!(-20 == (-15).round_to(10, Tie::TowardEven));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::TowardEven));
    /// assert!(10 == 6.round_to(10, Tie::TowardEven));
    /// assert!(0 == (-4).round_to(10, Tie::TowardEven));
    /// assert!(-10 == (-6).round_to(10, Tie::TowardEven));
    /// ```
    TowardEven,

    /// Round half toward odd.
    ///
    /// “Odd” has a special meaning here since we only care about round values.
    /// If we are rounding to the nearest 10, then 0 is even, 10 is odd, 20 is
    /// even, and so on.
    ///
    /// If we are rounding to whole numbers, then even and odd have the
    /// conventional meaning.
    ///
    /// In general, a multiple of factor _n_ is odd if `(n / factor) % 2 != 0`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(10 == 15.round_to(10, Tie::TowardOdd));
    /// assert!(10 == 5.round_to(10, Tie::TowardOdd));
    /// assert!(-10 == (-5).round_to(10, Tie::TowardOdd));
    /// assert!(-10 == (-15).round_to(10, Tie::TowardOdd));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::TowardOdd));
    /// assert!(10 == 6.round_to(10, Tie::TowardOdd));
    /// assert!(0 == (-4).round_to(10, Tie::TowardOdd));
    /// assert!(-10 == (-6).round_to(10, Tie::TowardOdd));
    /// ```
    TowardOdd,
}
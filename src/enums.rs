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

pub enum Tie<Integer> {
    Ignore(Integer),
    Break(Integer),
    /// Banker's Rounding
    Even,
    Odd,
}

pub enum OneSidedInteger {
    /// Pushes the number higher.
    /// Acts as `Up`, `Ceil`, and `AwayFromZero`.
    Higher,

    /// Pulls the number lower.
    /// Acts as `Down`, `Floor`, `Truncate`, and `ToZero`.
    Lower,
}

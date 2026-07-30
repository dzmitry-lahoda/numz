

- more categorical and const num-traits and rounding enums
- works on Copy only and will always only on these
- non zero support
- designed for newnum(newtype patter) support
  - so that `a(u64) x b(u32) = c(u96)` if needed
  - or `nonzero / nonzero = zeroable`

## because 

- https://github.com/rust-num/num-traits/pull/346
- https://github.com/rust-num/num-traits/issues?q=label%3A%22breaking%20change%22%20
- https://github.com/rust-num/num-traits/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22breaking%20change%22
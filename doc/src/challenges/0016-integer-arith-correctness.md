# Challenge 16: Correctness of primitive integer types' arithmetic functions

- **Status:** Open
- **Tracking Issue:** [#71](https://github.com/model-checking/verify-rust-std/issues/71)
- **Start date:** *2025/02/24*
- **End date:** *2025/08/24*
- **Reward:** *N/A*

-------------------

## Goal

Verify the correctness of primitive integer types' arithmetic functions in `core::num::mod.rs`.

For this challenge, you can assume that all the intrinsic functions are correct.


### Success Criteria

Prove the correctness the following functions and methods in `core::num::mod.rs` for all primitive integer types
`{isize, i8, i16, i32, i64, i128 , usize, u8, u16, u32, u64, u128}`

| Functions |
|--------- |
|  `checked_add`   |
|  `saturating_add`  |
|  `unchecked_add`   |
|  `overflowing_add` |
|  `wrapping_add`   |

And similar versions for `sub`, `mul`, `abs`, `neg`, `pow`, `rem`, `div` (if available).

| More functions |
|--------- |
|  `pow`   |
|  `rem_euclid`  |
|  `div_euclid`   |
|  `div_ceil` |
|  `div_floor`   |
|  `ilog2`   |
|  `ilog10`   |


Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

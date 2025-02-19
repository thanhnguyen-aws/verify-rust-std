# Challenge 24: Correctness of primitive integer types' bit functions

- **Status:** Open
- **Tracking Issue:** [#71](https://github.com/model-checking/verify-rust-std/issues/71)
- **Start date:** *2025/02/24*
- **End date:** *2025/08/24*
- **Reward:** *N/A*

-------------------

## Goal

Verify the correctness of primitive integer types' functions in `core::num::mod.rs`.


### Success Criteria

Prove the correctness the following functions and methods in `core::num::mod.rs` for all primitive integer types
`{isize, i8, i16, i32, i64, i128 , usize, u8, u16, u32, u64, u128}`

| Function |
|--------- |
|  `checked_shl`   |
|  `saturating_shl`  |
|  `unchecked_shl`   |
|  `overflowing_shl` |
|  `wrapping_shl`   |
|  `unbounded_shr`   |
|  `checked_shr`   |
|  `saturating_shr`  |
|  `unchecked_shr`   |
|  `overflowing_shr` |
|  `wrapping_shr`   |
|  `unbounded_shr`   |
|  `swap_bytes`|
|  `reverse_bits`|
|  `rotate_left`|
|  `rotate_right`|
| `to_be` |
| `to_le` |
| `to_be_bytes` |
| `to_le_bytes` |
| `trailing_zeros` |
| `trailing_ones` |
| `leading_zeros` |
| `leading_ones` |
| `count_zeros` |
| `count_ones` |


Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

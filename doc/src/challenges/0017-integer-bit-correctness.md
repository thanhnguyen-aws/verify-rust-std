# Challenge 17: Correctness of primitive integer types' bit functions

- **Status:** Open
- **Tracking Issue:** [#71](https://github.com/model-checking/verify-rust-std/issues/71)
- **Start date:** *2025/03/07*
- **End date:** *2025/10/17*
- **Reward:** *N/A*

-------------------

## Goal

Verify the correctness of primitive integer types' functions in `core::num::mod.rs`.

For this challenge, you can assume that all the intrinsic functions are correct.

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


### List of UBs

In addition to any properties called out as `SAFETY` comments in the source
code,
all proofs must automatically ensure the absence of the following [undefined behaviors](https://github.com/rust-lang/reference/blob/142b2ed77d33f37a9973772bd95e6144ed9dce43/src/behavior-considered-undefined.md):

* Accessing (loading from or storing to) a place that is dangling or based on a misaligned pointer.
* Reading from uninitialized memory.
* Mutating immutable bytes.
* Producing an invalid value

Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

# Challenge 19: Correctness of `NonZero` functions

- **Status:** Open
- **Tracking Issue:** [#71](https://github.com/model-checking/verify-rust-std/issues/71)
- **Start date:** *2025/03/07*
- **End date:** *2025/10/17*
- **Reward:** *N/A*

-------------------

## Goal

Verify the correctness of `NonZero` functions in `core::num`.

### Assumptions

This challenge is the continuation of Challenge 12: Safety of `NonZero` and Challenge 14: Safety of Primitive Conversions.

Now, you need to verify the "correctness" of the functions listed in Challenge 12.

HOWEVER, You DON'T need to prove the "correctness" from the functions' descriptions, you JUST need to prove that those functions are consistent with those of 
the primitive integer types (under safety preconditions of Challenge 12).

For example, for the `max` function, you need to prove that 
`∀ T in {isize, i8, i16, ... , usize, u8, ... },  ∀ x, y : NonZero<T>, x.max(y).get() == x.get().max(y.get())`

Proving the correctness of the functions of primitive integer types is proposed in Challenge 16 and 17.

### Success Criteria

Verify that the following functions and methods (all located within `core::num::nonzero`) are consistent with those of all of the primitive integer types:

| Function |
|--------- |
|  `max`   |
|  `min`   |
|  `clamp`   |
|  `bitor`  (all 3 implementations) |
|  `count_ones`   |
|  `rotate_left`   |
|  `rotate_right`   |
|  `swap_bytes`   |
|  `reverse_bits`   |
|  `from_be`   |
|  `from_le`   |
|  `to_be`   |
|  `to_le`   |
|  `checked_mul`   |
|  `saturating_mul`   |
|  `unchecked_mul`   |
|  `checked_pow`   |
|  `saturating_pow`   |
|  `neg`   |
|  `checked_add`   |
|  `saturating_add`   |
|  `unchecked_add`   |
|  `checked_next_power_of_two`   |
|  `midpoint`   |
|  `isqrt`   |
|  `abs`   |
|  `checked_abs`   |
|  `overflowing_abs`   |
|  `saturating_abs`   |
|  `wrapping_abs`   |
|  `unsigned_abs`   |
|  `checked_neg`   |
|  `overflowing_neg`   |
|  `wrapping_neg` |
|  `from_mut`   |
|  `from_mut_unchecked` |


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


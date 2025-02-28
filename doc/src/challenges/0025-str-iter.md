# Challenge 25: Correctness of str functions

- **Status:** Open
- **Tracking Issue:** [#71](https://github.com/model-checking/verify-rust-std/issues/71)
- **Start date:** *2025/03/07*
- **End date:** *2025/10/17*
- **Reward:** *N/A*

-------------------

## Goal

Verify the correctness of functions in `core::src::str::mod.rs`.

IMPORTANT NOTE: You can assume the correctness of all the functions that are verified in Challenges 21 to 24, BUT you may have to verify the correctness of other dependent functions in core::str::iter

### Success Criteria

Prove the correctness the following functions and methods in `core::str::mod.rs` :
| Function |
|--------- |
| `is_char_boundary`|
| `floor_char_boundary`|
| `ceil_char_boundary`|
|  `split_at`   |
|  `split_at_mut`  |
|  `split_at_checked`   |
|  `split_at_mut_checked` |
|  `split_at_unchecked`   |
|  `split_at_mut_unchecked`   |
|  `split_whitespace`   |
|  `split_ascii_whitespace`  |
|  `lines`   |
|  `lines_any` |
|  `starts_with`   |
|  `ends_with`   |
|  `find`|
|  `rfind`|
|  `rotate_left`|
|  `rotate_right`|
| `split` |
| `split_inclusive` |
| `rsplit` |
| `split_terminator` |
| `rsplit_terminator` |
| `splitn` |
| `rsplitn` |
| `split_once` |
| `rsplit_once` |
| `matches` |
| `rmatches` |
| `match_indices` |
| `rmatch_indices` |
| `trim` |
| `trim_start` |
| `trim_end` |
| `trim_left` |
| `trim_right` |
| `trim_right` |
| `trim_start_matches` |
| `strip_prefix` |
| `strip_suffix` |
| `trim_end_matches` |
| `trim_left_matches` |
| `trim_right_matches` |

The verification must be unbounded---it must hold for str of arbitrary size.


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

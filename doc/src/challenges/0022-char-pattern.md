# Challenge 23: Verify the correctness of char-related functions in str::pattern

- **Status:** Open
- **Tracking Issue:** [#29](https://github.com/model-checking/verify-rust-std/issues/29)
- **Start date:** *2025/03/07*
- **End date:** *2025/10/17*
- **Reward:** *5,000 USD*

-------------------


### Details

Most functions in str library is defined on the concept of Pattern, which can be a char, a set of chars, a char filter, or a substring.
Those functions is implemented in core::src::str, but the core of them is implemented in core::src::str::pattern.
The main purposes of the functions in core::src::str::pattern is converting a str (a slice of bytes) into some kinds of Searchers (iterators), 
then implementing the searching alorithm for those Searchers.

IMPORTANT NOTE: You can assume the correctness of functions in Challenge 21.

### Success Criteria

Verify the memory safety and functional correctness of the following functions in
https://github.com/rust-lang/rust/blob/96cfc75584359ae7ad11cc45968059f29e7b44b7/library/core/src/str/pattern.rs

1. `next`, `next_match`, `next_back`, `next_match_back`, 
which are implemented for `CharSearcher`, `MultiCharEqSearcher`, `CharArraySearcher` , `CharArrayRefSearcher`, `CharSliceSearcher`, `CharPredicateSearcher` 
2. `is_contained_in`, `is_prefix_of`, `strip_prefix_of`
which are implemented for `char`, `[char; N]`, `&[char; N]`, `&[char]`, `FnMut(char) -> bool`

The verification must be unbounded---it must hold for inputs of arbitrary size.


### List of UBs

All proofs must automatically ensure the absence of the following undefined behaviors [ref](https://github.com/rust-lang/reference/blob/142b2ed77d33f37a9973772bd95e6144ed9dce43/src/behavior-considered-undefined.md):

* Accessing (loading from or storing to) a place that is dangling or based on a misaligned pointer.
* Reading from uninitialized memory except for padding or unions.
* Mutating immutable bytes.
* Producing an invalid value


Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

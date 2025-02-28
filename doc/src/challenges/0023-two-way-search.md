# Challenge 23: Verify the correctness of the Two-Way search algorithm implementation in str::pattern

- **Status:** Open
- **Tracking Issue:** [#29](https://github.com/model-checking/verify-rust-std/issues/29)
- **Start date:** *2025/03/07*
- **End date:** *2025/10/17*
- **Reward:** *5,000 USD*

-------------------

### Details

This algorithm is the main component of implementing searching functions for Patterns as substrings.

### Success Criteria

Verify the memory safety and functional correctness of the Two-Way search algorithm implementation in str::pattern.


### List of UBs

All proofs must automatically ensure the absence of the following undefined behaviors [ref](https://github.com/rust-lang/reference/blob/142b2ed77d33f37a9973772bd95e6144ed9dce43/src/behavior-considered-undefined.md):

* Accessing (loading from or storing to) a place that is dangling or based on a misaligned pointer.
* Reading from uninitialized memory except for padding or unions.
* Mutating immutable bytes.
* Producing an invalid value


Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

# Challenge 21: Verify the memory safety and functional correctness of `slice` iter functions

- **Status:** Open
- **Tracking Issue:** [#29](https://github.com/model-checking/verify-rust-std/issues/29)
- **Start date:** *2025/03/07*
- **End date:** *2025/10/17*
- **Reward:** *?*

-------------------


## Goal

Verify the memory safety of functional correctness of [`std::slice` iter functions] (https://github.com/rust-lang/rust/blob/c290e9de32e8ba6a673ef125fde40eadd395d170/library/core/src/slice/iter/macros.rs).


### Success Criteria

The memory safety of the following public functions that iterating over the internal inductive data type must be verified:

| Function |
|---------|
|next_back_unchecked| 
|make_slice| 
|pre_dec_end|
|post_inc_start| 
|len|
|is_empty|
|next|
|size_hint| 
|count| 
|nth| 
|advance_by| 
|last| 
|fold| 
|for_each| 
|all| 
|any| 
|find| 
|find_map| 
|position| 
|rposition| 
|next_back| 
|nth_back| 
|advance_back_by| 
|next_unchecked| 


The verification must be unbounded---it must hold for slices of arbitrary length.

It is OK to assume that the generic type `T` of the proofs is primitive types, e.g., `i32`, `u32`, `bool`, etc.

### List of UBs

All proofs must automatically ensure the absence of the following undefined behaviors [ref](https://github.com/rust-lang/reference/blob/142b2ed77d33f37a9973772bd95e6144ed9dce43/src/behavior-considered-undefined.md):

* Accessing (loading from or storing to) a place that is dangling or based on a misaligned pointer.
* Reading from uninitialized memory except for padding or unions.
* Mutating immutable bytes.
* Producing an invalid value


Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

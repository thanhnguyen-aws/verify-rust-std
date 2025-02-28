# Challenge 5: Verify the correctness of UTF8 and UTF16 encoding functions
- **Status:** Open
- **Tracking Issue:** [#29](https://github.com/model-checking/verify-rust-std/issues/29)
- **Start date:** *2025/03/07*
- **End date:** *2025/10/17*
- **Reward:** *5,000 USD*

-------------------


## Goal

Verify the the correctness of functions related to UTF8 and UTF16 encoding

### Details

Rust str and String are either UTF8 and UTF16 encoded. Verifying the correctess of the related functions is important in ensuring the safety and correctness of Rust programs that involve Strings.


### Success Criteria

Verify the the correctness of the following functions is functionally correct according to the UTF8 anf UTF16 descriptions in:
https://en.wikipedia.org/wiki/UTF-8 and https://en.wikipedia.org/wiki/UTF-16

| Function | Location |
|---------|---------|
|run_utf8_validation| core::src::str::validation |
|next_code_point| core::src::str::validation |
|next_code_point_reverse| core::src::str::validation |
|decode_utf16| core::src::char::decode |
|from_utf8| core::str::converts |
|from_utf8_unchecked| core::str::converts |
|from_utf8_mut| core::str::converts |
|from_utf8_unchecked_mut| core::str::converts |
|chars| core::str::mod |
|char_indices| core::str::mod |
|encode_utf16| core::str::mod|
|from_utf16| alloc::src::string |
|from_utf16_lossy| alloc::src::string |



The verification must be unbounded---it must hold for inputs of arbitrary size.



### List of UBs

All proofs must automatically ensure the absence of the following undefined behaviors [ref](https://github.com/rust-lang/reference/blob/142b2ed77d33f37a9973772bd95e6144ed9dce43/src/behavior-considered-undefined.md):

* Accessing (loading from or storing to) a place that is dangling or based on a misaligned pointer.
* Reading from uninitialized memory except for padding or unions.
* Mutating immutable bytes.
* Producing an invalid value


Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

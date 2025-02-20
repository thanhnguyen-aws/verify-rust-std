# Challenge 18: Correctness of instrinsic floating-point mathematic functions

- **Status:** Open
- **Tracking Issue:** [#71](https://github.com/model-checking/verify-rust-std/issues/71)
- **Start date:** *2025/02/24*
- **End date:** *2025/08/24*
- **Reward:** *N/A*

-------------------

## Goal

Floating-point computation is subjected to rounding errors. Knowing the accuracy of basic library functions is crucial when using them in precision-critical computations.  

Ideally, we expect the elementary functions `cosf16` to be correctly-rounded (the unrounded value of the real mathematic functions should be rounded to the returned value of the floating-point functions
under all rounding-modes). However, it is not always the case.

In this challenge, for each function listed in the next section, you are required to either:
- Prove that the function is correctly-rounded.
OR
- Provide an input to show that the function is not correctly-rounded, then provide a "tight" rigorous bound for the relative error of that function. 
A bound is considered "tight" if you can provide an input such that the error is at least 90% the magnitude of the bound.


### Success Criteria

Prove thet the following functions are correctly-rounded or provide "tight" error-bounds for them.

| Functions |
|--------- |
|  `cosf16`   |
|  `cosf32`  |
|  `cosf64`   |
|  `cosf128` |


And similar versions for `exp2`, `log2`, `log10`, `powf`, `sinf`, `sqrtf`, 


Note: All solutions to verification challenges need to satisfy the criteria established in the [challenge book](../general-rules.md)
in addition to the ones listed above.

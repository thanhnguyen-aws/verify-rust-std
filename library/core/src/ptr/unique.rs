use safety::{ensures, requires};

use crate::fmt;
#[cfg(kani)]
use crate::kani;
use crate::marker::{PhantomData, Unsize};
use crate::ops::{CoerceUnsized, DispatchFromDyn};
use crate::pin::PinCoerceUnsized;
use crate::ptr::NonNull;

/// A wrapper around a raw non-null `*mut T` that indicates that the possessor
/// of this wrapper owns the referent. Useful for building abstractions like
/// `Box<T>`, `Vec<T>`, `String`, and `HashMap<K, V>`.
///
/// Unlike `*mut T`, `Unique<T>` behaves "as if" it were an instance of `T`.
/// It implements `Send`/`Sync` if `T` is `Send`/`Sync`. It also implies
/// the kind of strong aliasing guarantees an instance of `T` can expect:
/// the referent of the pointer should not be modified without a unique path to
/// its owning Unique.
///
/// If you're uncertain of whether it's correct to use `Unique` for your purposes,
/// consider using `NonNull`, which has weaker semantics.
///
/// Unlike `*mut T`, the pointer must always be non-null, even if the pointer
/// is never dereferenced. This is so that enums may use this forbidden value
/// as a discriminant -- `Option<Unique<T>>` has the same size as `Unique<T>`.
/// However the pointer may still dangle if it isn't dereferenced.
///
/// Unlike `*mut T`, `Unique<T>` is covariant over `T`. This should always be correct
/// for any type which upholds Unique's aliasing requirements.
#[unstable(
    feature = "ptr_internals",
    issue = "none",
    reason = "use `NonNull` instead and consider `PhantomData<T>` \
              (if you also use `#[may_dangle]`), `Send`, and/or `Sync`"
)]
#[doc(hidden)]
#[repr(transparent)]
// Lang item used experimentally by Miri to define the semantics of `Unique`.
#[lang = "ptr_unique"]
pub struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    // NOTE: this marker has no consequences for variance, but is necessary
    // for dropck to understand that we logically own a `T`.
    //
    // For details, see:
    // https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#phantom-data
    _marker: PhantomData<T>,
}

/// `Unique` pointers are `Send` if `T` is `Send` because the data they
/// reference is unaliased. Note that this aliasing invariant is
/// unenforced by the type system; the abstraction using the
/// `Unique` must enforce it.
#[unstable(feature = "ptr_internals", issue = "none")]
unsafe impl<T: Send + ?Sized> Send for Unique<T> {}

/// `Unique` pointers are `Sync` if `T` is `Sync` because the data they
/// reference is unaliased. Note that this aliasing invariant is
/// unenforced by the type system; the abstraction using the
/// `Unique` must enforce it.
#[unstable(feature = "ptr_internals", issue = "none")]
unsafe impl<T: Sync + ?Sized> Sync for Unique<T> {}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: Sized> Unique<T> {
    /// Creates a new `Unique` that is dangling, but well-aligned.
    ///
    /// This is useful for initializing types which lazily allocate, like
    /// `Vec::new` does.
    ///
    /// Note that the pointer value may potentially represent a valid pointer to
    /// a `T`, which means this must not be used as a "not yet initialized"
    /// sentinel value. Types that lazily allocate must track initialization by
    /// some other means.
    #[must_use]
    #[inline]
    pub const fn dangling() -> Self {
        // FIXME(const-hack) replace with `From`
        Unique { pointer: NonNull::dangling(), _marker: PhantomData }
    }
}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> Unique<T> {
    /// Creates a new `Unique`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    #[requires(!ptr.is_null())]
    #[ensures(|result| result.as_ptr() == ptr)]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        unsafe { Unique { pointer: NonNull::new_unchecked(ptr), _marker: PhantomData } }
    }

    /// Creates a new `Unique` if `ptr` is non-null.
    #[inline]
<<<<<<< HEAD
    #[rustc_const_unstable(feature = "const_align_offset", issue = "90962")]
    #[ensures(|result| result.is_none() == ptr.is_null())]
    #[ensures(|result| result.is_none() || result.unwrap().as_ptr() == ptr)]
=======
>>>>>>> 1ce7810500c125b4cba495211f908db2a9b1f1cb
    pub const fn new(ptr: *mut T) -> Option<Self> {
        if let Some(pointer) = NonNull::new(ptr) {
            Some(Unique { pointer, _marker: PhantomData })
        } else {
            None
        }
    }

    /// Acquires the underlying `*mut` pointer.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    #[ensures(|result| !result.is_null())]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer.as_ptr()
    }

    /// Acquires the underlying `*mut` pointer.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    #[ensures(|result| result.as_ptr() == self.pointer.as_ptr())]
    pub const fn as_non_null_ptr(self) -> NonNull<T> {
        self.pointer
    }

    /// Dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&*my_ptr.as_ptr()`.
    #[must_use]
    #[inline]
    pub const unsafe fn as_ref(&self) -> &T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { self.pointer.as_ref() }
    }

    /// Mutably dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&mut *my_ptr.as_ptr()`.
    #[must_use]
    #[inline]
    pub const unsafe fn as_mut(&mut self) -> &mut T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a mutable reference.
        unsafe { self.pointer.as_mut() }
    }

    /// Casts to a pointer of another type.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub const fn cast<U>(self) -> Unique<U> {
        // FIXME(const-hack): replace with `From`
        // SAFETY: is `NonNull`
        Unique { pointer: self.pointer.cast(), _marker: PhantomData }
    }
}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> Clone for Unique<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> Copy for Unique<T> {}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized, U: ?Sized> CoerceUnsized<Unique<U>> for Unique<T> where T: Unsize<U> {}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Unique<U>> for Unique<T> where T: Unsize<U> {}

#[unstable(feature = "pin_coerce_unsized_trait", issue = "123430")]
unsafe impl<T: ?Sized> PinCoerceUnsized for Unique<T> {}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> fmt::Debug for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> fmt::Pointer for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> From<&mut T> for Unique<T> {
    /// Converts a `&mut T` to a `Unique<T>`.
    ///
    /// This conversion is infallible since references cannot be null.
    #[inline]
    fn from(reference: &mut T) -> Self {
        Self::from(NonNull::from(reference))
    }
}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> From<NonNull<T>> for Unique<T> {
    /// Converts a `NonNull<T>` to a `Unique<T>`.
    ///
    /// This conversion is infallible since `NonNull` cannot be null.
    #[inline]
    fn from(pointer: NonNull<T>) -> Self {
        Unique { pointer, _marker: PhantomData }
    }
}

#[cfg(kani)]
#[unstable(feature = "kani", issue = "none")]
mod verify {
    use super::*;

    // pub const unsafe fn new_unchecked(ptr: *mut T) -> Self
    #[kani::proof_for_contract(Unique::new_unchecked)]
    pub fn check_new_unchecked() {
        let mut x: i32 = kani::any();
        let xptr = &mut x;
        unsafe {
            let _ = Unique::new_unchecked(xptr as *mut i32);
        }
    }

    // pub const fn new(ptr: *mut T) -> Option<Self>
    #[kani::proof_for_contract(Unique::new)]
    pub fn check_new() {
        let mut x: i32 = kani::any();
        let xptr = &mut x;
        let _ = Unique::new(xptr as *mut i32);
    }

    // pub const fn as_ptr(self) -> *mut T
    #[kani::proof_for_contract(Unique::as_ptr)]
    pub fn check_as_ptr() {
        let mut x: i32 = kani::any();
        let xptr = &mut x;
        unsafe {
            let unique = Unique::new_unchecked(xptr as *mut i32);
            assert_eq!(unique.as_ptr(), xptr as *mut i32);
        }
    }

    // pub const fn as_non_null_ptr(self) -> NonNull<T>
    #[kani::proof_for_contract(Unique::as_non_null_ptr)]
    pub fn check_as_non_null_ptr() {
        let mut x: i32 = kani::any();
        let xptr = &mut x;
        unsafe {
            let unique = Unique::new_unchecked(xptr as *mut i32);
            let _ = unique.as_non_null_ptr();
        }
    }

    // pub const unsafe fn as_ref(&self) -> &T
    #[kani::proof]
    pub fn check_as_ref() {
        let mut x: i32 = kani::any();
        let xptr = &mut x;
        unsafe {
            let unique = Unique::new_unchecked(xptr as *mut i32);
            assert_eq!(*unique.as_ref(), x);
        }
    }

    // pub const unsafe fn as_mut(&mut self) -> &mut T
    #[kani::proof]
    pub fn check_as_mut() {
        let mut x: i32 = kani::any();
        let xptr = &mut x;
        unsafe {
            let mut unique = Unique::new_unchecked(xptr as *mut i32);
            assert_eq!(*unique.as_mut(), x);
        }
    }

    // pub const fn cast<U>(self) -> Unique<U>
    #[kani::proof]
    pub fn check_cast() {
        let mut x: i32 = kani::any();
        let xptr = &mut x;
        unsafe {
            let unique = Unique::new_unchecked(xptr as *mut i32);
            assert_eq!(*unique.cast::<u32>().as_ref(), x as u32);
        }
    }
}

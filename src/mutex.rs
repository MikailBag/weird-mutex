use core::pin::Pin;

/// Wrapper that exchanges shared access for
/// `impl Sync`.
/// # Correctness
/// ## Explanation 1
/// This is essentially
/// [`std::sync::Mutex`](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html),
/// but with strictly smaller API surface (i.e. no `lock()`
/// method is provided).
/// ## Explanation 2
/// T being !Sync means that all references to each particular
/// T instance must be owned by the same thread.
/// To uphold this requirements, `WeirdMutex` does not allow
/// projecting `&WeirdMutex<T>` to &T. Only way to get a `&T`
/// is to use `get` method which mutably borrows WeirdMutex.
/// That way Rust aliasing rules ensure that only one thread
/// can obtain a `&T` from the WeirdMutex directly.
/// This thread can then make copies of this references, but
/// `T: !Sync` implies `&T: !Send`, so this thread can not
/// accidentaly send this references to other thread.
/// # Performance
/// This type is just a wrapper and should have no effect
/// on performance.
/// # Drawbacks
/// Since WeirdMutex disallows projecting shared references,
/// many standard trait implementations are very limited:
///  - `Debug` implementation does not show contained value
///  - Neither `Clone` nor `Copy` are not implemented
///  - `*Ord`, `*Eq`, `Hash` are not implemented too
/// If you have mutable access to WeirdMutex, you can always
/// call `get()` and call desired trait method on `&T` directly.
///
/// On the other hand WeirdMutex implements some traits where all
/// methods take `self` or `&mut self`.
// No derives here. All traits must be implemented manually in `impls.rs`.
pub struct WeirdMutex<T>(T);
// derived Send implementation is OK
unsafe impl<T> Sync for WeirdMutex<T> {}

impl<T> WeirdMutex<T> {
    /// Wraps a value in the WeirdMutex
    pub fn new(value: T) -> WeirdMutex<T> {
        Self(value)
    }

    /// The primary way to access stored value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Pinned `get_mut` counterpart.
    pub fn pinned_get_mut(self: Pin<&mut Self>) -> Pin<&mut T> {
        // SAFETY: mapping function does not move anything
        // we could use `pin-project-lite` instead of this unsafe,
        // but it doesn't worth longer compile time.
        unsafe { self.map_unchecked_mut(|this| &mut this.0) }
    }

    /// Convenience wrapper for `get_mut` which immediately
    /// casts `&mut T` to `&T` for cases when you don't need
    /// mutable access to inner value
    pub fn get(&mut self) -> &T {
        &self.0
    }

    /// Unchecked shared getter.
    /// # Safety
    /// Calling code must manually uphold invariant
    /// that only one thread can have references to stored value.
    ///
    /// Violating this requirement may result in undefined behavior
    /// depending on concrete `T`.
    pub unsafe fn get_unchecked(&self) -> &T {
        &self.0
    }
}

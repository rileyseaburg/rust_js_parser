use std::ops::{Deref, DerefMut};

/// A wrapper to make a type `Send`.
///
/// # Safety
///
/// The caller must ensure that the wrapped type is only accessed from the thread
/// that created it.
pub struct SendWrapper<T>(T);

impl<T> SendWrapper<T> {
    /// Creates a new `SendWrapper`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the wrapped type is only accessed from the
    /// thread that created it.
    pub unsafe fn new(t: T) -> Self {
        Self(t)
    }
}

unsafe impl<T> Send for SendWrapper<T> {}

impl<T> Deref for SendWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SendWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

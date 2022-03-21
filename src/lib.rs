//! Fallible allocation.

#![cfg_attr(nightly, feature(try_reserve_kind))]

use std::alloc::Layout;
use std::collections::TryReserveError;
use std::error::Error;
use std::fmt;

/// The error type for allocation failure.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AllocError(Layout);

impl AllocError {
    /// Creates a new `AllocError`.
    ///
    /// If the size of `layout` is zero, it means we do not know what the size is.
    #[must_use]
    #[inline]
    pub const fn new(layout: Layout) -> Self {
        AllocError(layout)
    }

    /// Returns the memory layout of the `AllocError`.
    #[must_use]
    #[inline]
    pub const fn layout(self) -> Layout {
        self.0
    }
}

impl Error for AllocError {}

impl fmt::Display for AllocError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.layout().size() != 0 {
            write!(
                f,
                "failed to allocate memory by required layout {{size: {}, align: {}}}",
                self.0.size(),
                self.0.align()
            )
        } else {
            write!(f, "failed to allocate memory")
        }
    }
}

#[cfg(not(nightly))]
impl From<TryReserveError> for AllocError {
    #[inline]
    fn from(_e: TryReserveError) -> Self {
        AllocError::new(unsafe { Layout::from_size_align_unchecked(0, 1) })
    }
}

#[cfg(nightly)]
impl From<TryReserveError> for AllocError {
    #[inline]
    fn from(e: TryReserveError) -> Self {
        use std::collections::TryReserveErrorKind;
        match e.kind() {
            TryReserveErrorKind::AllocError { layout, .. } => AllocError::new(layout),
            TryReserveErrorKind::CapacityOverflow => {
                unreachable!("unexpected capacity overflow")
            }
        }
    }
}

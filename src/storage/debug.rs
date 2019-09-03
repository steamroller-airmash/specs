
use std::fmt::{Debug, Formatter, Result};

/// An item that knows it's type name and can be debugged
pub trait DebugEntity: Debug {
  /// Gives a string representing the type name.
  fn type_name(&self) -> &'static str;
}

pub(crate) struct EntityDebugWrapper<'a, T>(pub &'a T);

trait MaybeDebug {
  fn maybe_fmt(&self, fmt: &mut Formatter) -> Result;
}

impl<'a, T> Debug for EntityDebugWrapper<'a, T> {
  fn fmt(&self, fmt: &mut Formatter) -> Result {
    self.0.maybe_fmt(fmt)
  }
}

impl<'a, T> DebugEntity for EntityDebugWrapper<'a, T> {
  fn type_name(&self) -> &'static str {
    std::any::type_name::<T>()
  }
}

#[cfg(feature = "nightly")]
mod trait_impl {
  use super::*;

  impl<T> MaybeDebug for T {
    default fn maybe_fmt(&self, fmt: &mut Formatter) -> Result {
      write!(fmt, "<no debug implementation>")
    }
  }

  impl<T: Debug> MaybeDebug for T {
    fn maybe_fmt(&self, fmt: &mut Formatter) -> Result {
      self.fmt(fmt)
    }
  }
}

#[cfg(not(feature = "nightly"))]
mod trait_impl {
  use super::*;

  impl<T> MaybeDebug for T {
    fn maybe_fmt(&self, fmt: &mut Formatter) -> Result {
      write!(fmt, "<must use `nightly` feature to support debug printing")
    }
  }
}

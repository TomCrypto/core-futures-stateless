#![no_std]
#![feature(generator_trait, optin_builtin_traits)]

#[doc(hidden)]
pub mod future;
#[doc(hidden)]
pub mod task;
pub use core::*;

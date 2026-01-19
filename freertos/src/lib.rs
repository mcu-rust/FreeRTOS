#![doc = include_str!("../README.md")]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg_attr(any(feature = "time", feature = "sync"), macro_use)]
extern crate alloc;

pub mod prelude;

mod assert_callback;
mod base_type;
mod shim;

#[cfg(feature = "allocator")]
mod allocator;
mod base;
#[cfg(feature = "sync")]
mod critical;
#[cfg(feature = "time")]
mod delays;
#[cfg(feature = "sync")]
mod event_group;
#[cfg(feature = "interrupt")]
mod isr;
#[cfg(feature = "sync")]
mod mutex;
#[cfg(cortex_m)]
mod os_trait_impls;
#[cfg(feature = "sync")]
mod queue;
#[cfg(feature = "sync")]
mod semaphore;
#[cfg(any(feature = "time", feature = "sync"))]
mod task;
#[cfg(feature = "time")]
mod timers;
#[cfg(any(feature = "time", feature = "sync"))]
mod units;
mod utils;

#[cfg(feature = "sync")]
pub mod patterns;

// Internal stuff that is only public for first Proof of Concept
pub use crate::base::*;
pub use crate::shim::*;
pub use os_trait::{self, os_type_alias};
// ----------

#[cfg(feature = "allocator")]
pub use crate::allocator::*;
pub use crate::assert_callback::*;
pub use crate::base::FreeRtosError;
#[cfg(feature = "sync")]
pub use crate::critical::*;
#[cfg(feature = "time")]
pub use crate::delays::*;
#[cfg(feature = "sync")]
pub use crate::event_group::*;
#[cfg(feature = "interrupt")]
pub use crate::isr::*;
#[cfg(feature = "sync")]
pub use crate::mutex::*;
#[cfg(cortex_m)]
pub use crate::os_trait_impls::*;
#[cfg(feature = "sync")]
pub use crate::queue::*;
#[cfg(feature = "sync")]
pub use crate::semaphore::*;
#[cfg(any(feature = "time", feature = "sync"))]
pub use crate::task::*;
#[cfg(feature = "time")]
pub use crate::timers::*;
#[cfg(any(feature = "time", feature = "sync"))]
pub use crate::units::*;
#[cfg(feature = "cpu-clock")]
pub use crate::utils::cpu_clock_hz;
pub use crate::utils::{shim_sanity_check, str_from_c_string};

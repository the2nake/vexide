//! Low level core functionality for [`vexide`](https://crates.io/crates/vexide).
//! The core crate is used in all other crates in the vexide ecosystem.
//!
//! Included in this crate:
//! - Competition state handling: [`competition`]
//! - Serial terminal printing: [`io`]
//! - Synchronization primitives: [`sync`]
//! - Program control: [`program`]

#![feature(never_type, asm_experimental_arch)]

extern crate alloc;

pub mod allocator;
pub mod backtrace;
pub mod competition;
pub mod critical_section;
pub mod io;
pub mod program;
pub mod sync;

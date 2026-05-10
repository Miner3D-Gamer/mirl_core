//! A crate with minimal functions that are shared all across mirl crates
//!
//! If possible, this will replace mirl
// Additions
// #![feature(f16)]
// #![feature(f128)]
#![feature(specialization)]
#![feature(slice_index_methods)]
// Const
#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_eval_select)]
#![feature(generic_const_exprs)]
// #![feature(const_ops)]
#![feature(const_destruct)]
// Accessing core
#![feature(core_intrinsics)]
#![feature(str_internals)]
#![feature(core_float_math)]
// Misc
#![feature(decl_macro)]
// #![feature(rustc_allow_const_fn_unstable)]
#![feature(rustc_attrs)]

#[cfg(feature = "std")]
/// Text related stuff
pub mod text;

/// Buffers to draw on
pub mod render;
#[cfg(feature = "std")]
pub use render::Buffer;
pub use render::ConstBuffer;

// /// Stuff that should exist by default yet doesn't; use `mirl::extensions::*;` to import all of 'em
// pub mod extensions;
/// Stuff related to graphics -> Color manipulation
///
/// For rendering use [`mirl::render`](crate::render)
pub mod graphics;

/// Platform specific/interactions like functions/structs
pub mod platform;

/// Some simple math operations
///
/// For geometry/expressions related stuff, look into `mirl_math`
pub mod math;

/// Directional stuff -> NESW, N NE E SE S SW W NW
pub mod directions;

/// Mostly just traits
pub mod prelude;

/// Stuff that hasn't been sorted yet
#[cfg(feature = "std")]
pub mod misc;

/// Useful constants -> std contains some of these internally yet doesn't expose them for anyone else to use
pub mod constants;

/// Re-export of all used dependencies
pub mod dependencies;

/// Macro defenitions
pub mod macros;

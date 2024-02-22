#![warn(missing_docs)]
//! # Context-Based Builders
//!
//! Build Rust objects based on shared [`Context`]. This is useful when you need to generate
//! multiple objects based on a set of similar properties, such as in preparation for unit
//! tests.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

mod context;
pub use context::{Context, MainContext, SubContext};
mod entry;
pub use entry::Entry;
mod impls;
pub mod prelude;

/// Trait to build an object based on a shared [`Context`]
pub trait Builder: Sized {
    /// Build a new object based on the [`Context`]
    fn build<C: Context>(ctx: &mut C) -> Self;
}

/// Trait to build an object based on a shared [`Context`] and name
pub trait NamedBuilder: Sized {
    /// Build a new object based on a static name and the [`Context`]
    fn build_with_name<C: Context>(ctx: &mut C, name: &'static str) -> Self;
}

type AnyMap = HashMap<(TypeId, Option<&'static str>), Box<dyn Any + Send + Sync>>;

/// Create a new [`MainContext`]
pub fn ctx() -> MainContext {
    MainContext::new()
}

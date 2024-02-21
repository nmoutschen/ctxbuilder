#![warn(missing_docs)]
//! # Context-Based Builders
//!
//! Build Rust objects based on shared [`Context`]. This is useful when you need to generate
//! multiple objects based on a set of similar properties, such as in preparation for unit
//! tests.

use std::{
    any::{Any, TypeId},
    collections::{hash_map, HashMap},
    marker::PhantomData,
};

mod impls;

/// Trait to build an object based on a shared [`Context`]
pub trait Builder: Sized {
    /// Build a new object based on the [`Context`]
    fn build(ctx: &mut Context) -> Self;
}

/// Trait to build an object based on a shared [`Context`] and name
pub trait NamedBuilder: Sized {
    /// Build a new object based on a static name and the [`Context`]
    fn build_with_name(ctx: &mut Context, name: &'static str) -> Self;
}

type AnyMap = HashMap<(TypeId, Option<&'static str>), Box<dyn Any + Send + Sync>>;

/// Shared context to build objects
#[derive(Default)]
pub struct Context {
    map: AnyMap,
}

impl Context {
    /// Create a new [`Context`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get an entry in the context by its type
    pub fn entry<T: Send + Sync + 'static>(&mut self) -> Entry<'_, T> {
        Entry::new(self.map.entry((TypeId::of::<T>(), None)))
    }

    /// Get an entry in the context by its name and type
    pub fn entry_named<T: Send + Sync + 'static>(&mut self, name: &'static str) -> Entry<'_, T> {
        Entry::new(self.map.entry((TypeId::of::<T>(), Some(name))))
    }

    /// Get an object by its type
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .get(&(TypeId::of::<T>(), None))
            .and_then(|boxed| (**boxed).downcast_ref())
    }

    /// Get an object by its name and type
    pub fn get_named<T: Send + Sync + 'static>(&self, name: &'static str) -> Option<&T> {
        self.map
            .get(&(TypeId::of::<T>(), Some(name)))
            .and_then(|boxed| (**boxed).downcast_ref())
    }

    /// Insert an object by type
    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .insert((TypeId::of::<T>(), None), Box::new(val))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    /// Insert an object by type and name
    pub fn insert_named<T: Send + Sync + 'static>(
        &mut self,
        name: &'static str,
        val: T,
    ) -> Option<T> {
        self.map
            .insert((TypeId::of::<T>(), Some(name)), Box::new(val))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }
}

type InnerEntry<'c> =
    hash_map::Entry<'c, (TypeId, Option<&'static str>), Box<dyn Any + Send + Sync>>;

/// View into a single entry in a [`Context`]
#[derive(Debug)]
pub struct Entry<'c, T> {
    inner: InnerEntry<'c>,
    _phantom_data: PhantomData<T>,
}

impl<'c, T> Entry<'c, T> {
    fn new(inner: InnerEntry<'c>) -> Self {
        Self {
            inner,
            _phantom_data: PhantomData,
        }
    }
}

impl<'c, T: Send + Sync + 'static> Entry<'c, T> {
    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable
    /// reference to the value in the entry
    pub fn or_insert(self, default: T) -> &'c mut T {
        self.inner
            .or_insert(Box::new(default))
            .downcast_mut()
            .expect("downcast_mut on T")
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'c mut T {
        self.inner
            .or_insert_with(|| Box::new(default()))
            .downcast_mut()
            .expect("downcast_mut on T")
    }

    /// Provides in-place mutable access to an occupied entry before any potential inserts into the
    /// [`Context`]
    pub fn and_modify<F: FnOnce(&mut T)>(self, f: F) -> Self {
        Entry::new(
            self.inner
                .and_modify(|v| f(v.downcast_mut().expect("downcast_mut on T"))),
        )
    }
}

impl<'c, T: Default + Send + Sync + 'static> Entry<'c, T> {
    /// Ensures a value is in the entry by inserting the default value if empty, and returns a
    /// mutable reference to the value in the entry
    pub fn or_default(self) -> &'c mut T {
        // We need to use `or_insert` here, because we need to build a `Box` for `T` specifically
        #[allow(clippy::unwrap_or_default)]
        self.inner
            .or_insert(Box::<T>::default())
            .downcast_mut()
            .expect("downcast_mut on T")
    }
}

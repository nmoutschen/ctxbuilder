use std::any::TypeId;

use crate::{AnyMap, Builder, Entry, NamedBuilder};

/// Trait for implementing a shared context to generate objects
pub trait Context: Sized {
    /// Get an entry in the context by its type
    fn entry<T: Send + Sync + 'static>(&mut self) -> Entry<'_, T>;

    /// Get an entry in the context by its name and type
    fn entry_named<T: Send + Sync + 'static>(&mut self, name: &'static str) -> Entry<'_, T>;

    /// Get an object by its type
    fn get<T: Send + Sync + 'static>(&self) -> Option<&T>;

    /// Get an object by its name and type
    fn get_named<T: Send + Sync + 'static>(&self, name: &'static str) -> Option<&T>;

    /// Insert an object by type
    fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T>;

    /// Insert an object by type and name
    fn insert_named<T: Send + Sync + 'static>(&mut self, name: &'static str, val: T) -> Option<T>;

    /// Convenience method to add objects by type while constructing the [`Context`]
    fn with<T: Send + Sync + 'static>(mut self, val: T) -> Self {
        self.insert(val);
        self
    }

    /// Convenience method to add objects by name and type while constructing the [`Context`]
    fn with_named<T: Send + Sync + 'static>(mut self, name: &'static str, val: T) -> Self {
        self.insert_named(name, val);
        self
    }

    /// Build a new object with this context
    fn build<T: Builder>(&mut self) -> T {
        T::build(self)
    }

    /// Build a new named object with this context
    fn build_named<T: NamedBuilder>(&mut self, name: &'static str) -> T {
        T::build_with_name(self, name)
    }
}

/// Shared context to build objects
#[derive(Default)]
pub struct MainContext {
    map: AnyMap,
}

impl MainContext {
    /// Create a new [`MainContext`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a [`SubContext`] from this context
    pub fn sub(&self) -> SubContext {
        SubContext {
            ctx: self,
            map: Default::default(),
        }
    }
}

impl Context for MainContext {
    fn entry<T: Send + Sync + 'static>(&mut self) -> Entry<'_, T> {
        Entry::new(None, self.map.entry((TypeId::of::<T>(), None)))
    }

    fn entry_named<T: Send + Sync + 'static>(&mut self, name: &'static str) -> Entry<'_, T> {
        Entry::new(None, self.map.entry((TypeId::of::<T>(), Some(name))))
    }

    fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .get(&(TypeId::of::<T>(), None))
            .and_then(|boxed| (**boxed).downcast_ref())
    }

    fn get_named<T: Send + Sync + 'static>(&self, name: &'static str) -> Option<&T> {
        self.map
            .get(&(TypeId::of::<T>(), Some(name)))
            .and_then(|boxed| (**boxed).downcast_ref())
    }

    fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .insert((TypeId::of::<T>(), None), Box::new(val))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    fn insert_named<T: Send + Sync + 'static>(&mut self, name: &'static str, val: T) -> Option<T> {
        self.map
            .insert((TypeId::of::<T>(), Some(name)), Box::new(val))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }
}

/// Sub-context that inherits from another context
pub struct SubContext<'c> {
    ctx: &'c MainContext,
    map: AnyMap,
}

impl<'c> Context for SubContext<'c> {
    fn entry<T: Send + Sync + 'static>(&mut self) -> Entry<'_, T> {
        Entry::new(self.ctx.get(), self.map.entry((TypeId::of::<T>(), None)))
    }

    fn entry_named<T: Send + Sync + 'static>(&mut self, name: &'static str) -> Entry<'_, T> {
        Entry::new(
            self.ctx.get_named(name),
            self.map.entry((TypeId::of::<T>(), Some(name))),
        )
    }

    fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .get(&(TypeId::of::<T>(), None))
            .and_then(|boxed| (**boxed).downcast_ref())
            .or_else(|| self.ctx.get())
    }

    fn get_named<T: Send + Sync + 'static>(&self, name: &'static str) -> Option<&T> {
        self.map
            .get(&(TypeId::of::<T>(), Some(name)))
            .and_then(|boxed| (**boxed).downcast_ref())
            .or_else(|| self.ctx.get_named(name))
    }

    fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .insert((TypeId::of::<T>(), None), Box::new(val))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    fn insert_named<T: Send + Sync + 'static>(&mut self, name: &'static str, val: T) -> Option<T> {
        self.map
            .insert((TypeId::of::<T>(), Some(name)), Box::new(val))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    fn with<T: Send + Sync + 'static>(mut self, val: T) -> Self {
        self.insert(val);
        self
    }

    fn with_named<T: Send + Sync + 'static>(mut self, name: &'static str, val: T) -> Self {
        self.insert_named(name, val);
        self
    }

    fn build<T: Builder>(&mut self) -> T {
        T::build(self)
    }

    fn build_named<T: NamedBuilder>(&mut self, name: &'static str) -> T {
        T::build_with_name(self, name)
    }
}

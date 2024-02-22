use std::{
    any::{Any, TypeId},
    collections::hash_map,
    marker::PhantomData,
};

type InnerEntry<'c> =
    hash_map::Entry<'c, (TypeId, Option<&'static str>), Box<dyn Any + Send + Sync>>;

/// View into a single entry in a context
#[derive(Debug)]
pub struct Entry<'c, T> {
    main: Option<&'c T>,
    inner: InnerEntry<'c>,
    _phantom_data: PhantomData<T>,
}

impl<'c, T> Entry<'c, T> {
    pub(crate) fn new(main: Option<&'c T>, inner: InnerEntry<'c>) -> Self {
        Self {
            main,
            inner,
            _phantom_data: PhantomData,
        }
    }
}

impl<'c, T: Send + Sync + 'static> Entry<'c, T> {
    /// Ensures a value is in the entry by inserting the default if empty, and returns a reference
    /// to the value in the entry
    pub fn or_insert(self, default: T) -> &'c T {
        match (self.main, self.inner) {
            // entry is vacant, but main contains something: return main
            (Some(main), InnerEntry::Vacant(_)) => main,
            // entry is occuped: return inner
            // main is empty: insert inner
            (_, inner) => inner
                .or_insert(Box::new(default))
                .downcast_ref()
                .expect("downcast_ref on T"),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a reference to the value in the entry
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'c T {
        match (self.main, self.inner) {
            // entry is vacant, but main contains something: return main
            (Some(main), InnerEntry::Vacant(_)) => main,
            // entry is occuped: return inner
            // main is empty: insert inner
            (_, inner) => inner
                .or_insert_with(|| Box::new(default()))
                .downcast_ref()
                .expect("downcast_ref on T"),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any potential inserts into the
    /// context
    pub fn and_modify<F: FnOnce(&mut T)>(self, f: F) -> Self {
        Entry::new(
            // Since `and_modify` ensures we transition into `Occupied` state, we don't need to
            // keep the inherited value.
            None,
            self.inner
                .and_modify(|v| f(v.downcast_mut().expect("downcast_mut on T"))),
        )
    }
}

impl<'c, T: Default + Send + Sync + 'static> Entry<'c, T> {
    /// Ensures a value is in the entry by inserting the default value if empty, and returns a
    /// reference to the value in the entry
    pub fn or_default(self) -> &'c T {
        // We need to use `or_insert` here, because we need to build a `Box` for `T` specifically
        #[allow(clippy::unwrap_or_default)]
        self.main.unwrap_or_else(|| {
            self.inner
                .or_insert(Box::<T>::default())
                .downcast_ref()
                .expect("downcast_ref on T")
        })
    }
}

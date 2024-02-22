#[allow(unused)]
use crate::{Builder, Context, NamedBuilder};

#[cfg(feature = "uuid")]
impl Builder for uuid::Uuid {
    fn build<C: Context>(ctx: &mut C) -> Self {
        *ctx.entry::<Self>().or_insert_with(uuid::Uuid::new_v4)
    }
}
#[cfg(feature = "uuid")]
impl NamedBuilder for uuid::Uuid {
    fn build_with_name<C: Context>(ctx: &mut C, name: &'static str) -> Self {
        *ctx.entry_named::<Self>(name)
            .or_insert_with(uuid::Uuid::new_v4)
    }
}

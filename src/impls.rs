#[allow(unused)]
use crate::{Builder, NamedBuilder};

#[cfg(feature = "uuid")]
impl Builder for uuid::Uuid {
    fn build(ctx: &mut crate::Context) -> Self {
        *ctx.entry::<Self>().or_insert_with(uuid::Uuid::new_v4)
    }
}
#[cfg(feature = "uuid")]
impl NamedBuilder for uuid::Uuid {
    fn build_with_name(ctx: &mut crate::Context, name: &'static str) -> Self {
        *ctx.entry_named::<Self>(name)
            .or_insert_with(uuid::Uuid::new_v4)
    }
}

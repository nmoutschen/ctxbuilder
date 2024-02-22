use ctxbuilder::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PetType {
    Dog,
    Cat,
}

#[test]
fn test_subcontext() {
    // GIVEN a MainContext with an existing value
    let ctx = ctxbuilder::ctx().with(PetType::Cat);

    // WHEN creating a subcontext
    let mut subctx = ctx.sub();

    // THEN we get the original value
    assert_eq!(subctx.get(), Some(&PetType::Cat));

    // WHEN inserting a different value in the subcontext
    subctx.insert(PetType::Dog);

    // THEN
    // * we get the original value from the subcontext
    // * the original context keeps the same value
    assert_eq!(subctx.get(), Some(&PetType::Dog));
    assert_eq!(ctx.get(), Some(&PetType::Cat));
}

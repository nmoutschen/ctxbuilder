#![allow(unused)]

use ctxbuilder::{Builder, Context};
use uuid::Uuid;

struct Person {
    id: Uuid,
}

impl Builder for Person {
    fn build<C: Context>(ctx: &mut C) -> Self {
        // Create a new ID for `Person`
        let id = *ctx.entry_named("person").or_insert_with(|| Uuid::new_v4());

        Self { id }
    }
}

struct Pet {
    owner: Uuid,
    pet_type: PetType,
    name: String,
}

impl Builder for Pet {
    fn build<C: Context>(ctx: &mut C) -> Self {
        let owner = *ctx.entry_named("person").or_insert_with(|| Uuid::new_v4());
        Self {
            owner,
            pet_type: PetType::build(ctx),
            name: "Rufus".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PetType {
    Dog,
    Cat,
}

impl Builder for PetType {
    fn build<C: Context>(ctx: &mut C) -> Self {
        *ctx.entry().or_insert(PetType::Dog)
    }
}

#[test]
fn test_builder_owner() {
    // GIVEN a single Context
    let mut ctx = ctxbuilder::ctx();

    // WHEN creating a `person` and `pet` from that Context
    let person: Person = ctx.build();
    let pet = Pet::build(&mut ctx);

    // THEN they use the same `person` ID
    assert_eq!(person.id, pet.owner);
}

#[test]
fn test_builder_pet_type() {
    // GIVEN a single Context that contains a PetType
    let mut ctx = ctxbuilder::ctx().with(PetType::Cat);

    // WHEN generating a new pet
    let pet = Pet::build(&mut ctx);

    // THEN it uses the specified pet type
    assert_eq!(pet.pet_type, PetType::Cat);
}

use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug)]
pub struct Cat {
    pub hungry: bool,
}

impl Cat {
    pub fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct AnimalWorld {
    cat: Cat,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for AnimalWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            cat: Cat { hungry: false },
        })
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
// #[given("a hungry cat")]
// #[given(regex = r"^a (hungry|satiated) cat$")]
#[given(expr = "a {word} cat")]
fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        s => panic!("expected 'hungry' or 'satiated', found: {}", s),
    }
}

// Don't forget to additionally `use cucumber::when;`.
#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

// Don't forget to additionally `use cucumber::then;`.
#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}

pub async fn run() {
    AnimalWorld::run("tests/features/example").await;
}

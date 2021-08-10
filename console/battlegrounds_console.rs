use battlegrounds_rs::loaders::card_loader::CardLoader;

use specs::prelude::*;

fn main() {
    let world = World::new();

    CardLoader::load(&world);
}

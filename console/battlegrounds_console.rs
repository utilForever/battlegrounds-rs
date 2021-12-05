use battlegrounds_rs::loaders::card_loader::CardLoader;

use legion::*;

fn main() {
    let world = World::default();

    CardLoader::load(&world);
}

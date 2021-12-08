use battlegrounds_rs::loaders::card_loader::CardLoader;

use legion::*;

fn main() {
    let mut world = World::default();

    CardLoader::load(&mut world);
}

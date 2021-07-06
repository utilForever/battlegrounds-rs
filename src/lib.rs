use specs::prelude::*;
use specs_derive::Component;

#[allow(dead_code)]
struct State {
    ecs: World
}

#[allow(dead_code)]
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_ecs() {
        use super::*;

        let mut gs = State {
            ecs: World::new()
        };
        gs.ecs.register::<Position>();

        gs.ecs.create_entity().with(Position {x: 40, y: 25 }).build();

        let positions = gs.ecs.read_storage::<Position>();
        for pos in (&positions).join() {
            assert_eq!(pos.x, 40);
            assert_eq!(pos.y, 25);
        }
    }
}

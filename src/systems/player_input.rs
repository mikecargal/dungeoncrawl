use crate::prelude::*;

lazy_static! {
    static ref MOVE_LEFT: Point = Point::new(-1, 0);
    static ref MOVE_RIGHT: Point = Point::new(1, 0);
    static ref MOVE_UP: Point = Point::new(0, -1);
    static ref MOVE_DOWN: Point = Point::new(0, 1);
    static ref DONT_MOVE: Point = Point::zero();
}
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = key {
        let mut did_something = false;

        let delta = match key {
            VirtualKeyCode::Left => *MOVE_LEFT,
            VirtualKeyCode::Right => *MOVE_RIGHT,
            VirtualKeyCode::Up => *MOVE_UP,
            VirtualKeyCode::Down => *MOVE_DOWN,
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();
                let mut items = <(Entity, &Item, &Point)>::query();
                items
                    .iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
                    .for_each(|(entity, _item, &_item_pos)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried { by: player });
                    });
                Point::zero()
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => *DONT_MOVE,
        };
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        if delta != *DONT_MOVE {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_entity, _item, carried)| carried.by == player_entity)
        .enumerate()
        .filter(|(item_count, _)| *item_count == n)
        .find_map(|(_, (item_entity, _item, _carried))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::zero()
}

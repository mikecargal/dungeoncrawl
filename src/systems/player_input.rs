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
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}

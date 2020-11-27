use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let destination = match rng.range(0, 9) {
            0 => Point::new(-1, -1),
            1 => Point::new(-1, 0),
            2 => Point::new(-1, 1),
            3 => Point::new(0, -1),
            4 => Point::new(0, 0),
            5 => Point::new(0, 1),
            6 => Point::new(1, -1),
            7 => Point::new(1, 0),
            _ => Point::new(1, 1),
        } + *pos;

        let mut attacked = false;
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(victim, _, _)| {
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },
                    ));
                }
                attacked = true;
            });

        if !attacked {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}

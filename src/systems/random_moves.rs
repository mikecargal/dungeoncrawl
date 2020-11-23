use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
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

        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
}

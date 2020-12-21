use crate::prelude::*;
use core::fmt;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    let player_pos = <(&Point, &Player)>::query().iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        DISTANCE_MAX_DEPTH,
    );

    let mut requested_destinations = HashSet::new();
    movers
        .iter(ecs)
        .filter(|(_, _, _, fov)| fov.is_visible(&player_pos))
        .for_each(|(entity, pos, _, _)| {
            let idx = map_idx(pos.x, pos.y);
            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
                };
                let attacked = positions
                    .iter(ecs)
                    .filter(|(victim, target_pos, _)| {
                        **target_pos == destination && victim_is_player(victim, ecs)
                    })
                    .inspect(|(victim, _, _)| {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: **victim,
                            },
                        ));
                    })
                    .count()
                    > 0;

                if !attacked && !requested_destinations.contains(&destination) {
                    requested_destinations.insert(destination);
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                }
            }
        });
}

fn victim_is_player(victim: &Entity, ecs: &SubWorld) -> bool {
    ecs.entry_ref(*victim)
        .unwrap()
        .get_component::<Player>()
        .is_ok()
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} )\n ( {} {} )", self.0, self.1, self.2, self.3)
    }
}

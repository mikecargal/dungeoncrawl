use super::MapArchitect;
use crate::prelude::*;

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        const MAX_DISTANCE_FROM_CENTER: f32 = 2000.0;

        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: vec![],
            monster_spawns: vec![],
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: None,
        };

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            self.drunkard(
                &Point::new(
                    rng.range(1, SCREEN_WIDTH - 1),
                    rng.range(1, SCREEN_HEIGHT - 1),
                ),
                rng,
                &mut mb.map,
            );

            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                DISTANCE_MAX_DEPTH,
            );
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &MAX_DISTANCE_FROM_CENTER)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        display(
            "Drunken Map ",
            &mb.map,
            &mb.player_start,
            &mb.amulet_start,
            &mb.monster_spawns,
        );
        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;
        while distance_staggered < STAGGER_DISTANCE {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            let stumble_pos = match rng.range(0, 6) {
                0 | 3 => Point {
                    x: drunkard_pos.x - 1,
                    y: drunkard_pos.y,
                },
                1 | 4 => Point {
                    x: drunkard_pos.x + 1,
                    y: drunkard_pos.y,
                },
                2 => Point {
                    x: drunkard_pos.x,
                    y: drunkard_pos.y - 1,
                },
                _ => Point {
                    x: drunkard_pos.x,
                    y: drunkard_pos.y + 1,
                },
            };
            if Map::in_floor_bounds(stumble_pos) {
                distance_staggered += 1;
                drunkard_pos = stumble_pos;
            }
        }
    }
}

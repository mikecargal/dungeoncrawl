use super::MapArchitect;
use crate::prelude::*;

const STAGGER_DISTANCE: usize = ((SCREEN_HEIGHT * SCREEN_WIDTH) / 5) as usize;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;
const MIN_AMULET_DISTANCE: f32 = (SCREEN_HEIGHT * SCREEN_WIDTH) as f32 / 25.0;
pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        const MAX_DISTANCE_FROM_CENTER: f32 = 2000.0;
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: vec![],
            monster_spawns: vec![],
            player_start: None,
            amulet_start: None,
            theme: None,
        };

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.player_start = Some(center);
        self.drunkard(&center, rng, &mut mb.map);
        while !map_is_complete(&mb) {
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
            mb.amulet_start = Some(mb.find_most_distant());
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb
    }
}

fn map_is_complete(mb: &MapBuilder) -> bool {
    if mb.amulet_start.is_none() {
        return false;
    }

    let floor_tile_count = mb
        .map
        .tiles
        .iter()
        .filter(|t| **t == TileType::Floor)
        .count();
    if floor_tile_count < DESIRED_FLOOR {
        return false;
    }

    let amulet_distance = mb.map.distance(
        mb.player_start.expect("No player!"),
        mb.amulet_start.unwrap(),
    );
    amulet_distance < MIN_AMULET_DISTANCE
}

impl DrunkardsWalkArchitect {
    pub fn new() -> Box<dyn MapArchitect> {
        Box::new(Self {})
    }

    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        for _ in 0..=STAGGER_DISTANCE {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            drunkard_pos = stumble(drunkard_pos, rng);
        }
    }
}

fn stumble(start_pos: Point, rng: &mut RandomNumberGenerator) -> Point {
    loop {
        let stumble_pos = match rng.range(0, 6) {
            0 | 3 => Point {
                x: start_pos.x - 1,
                y: start_pos.y,
            },
            1 | 4 => Point {
                x: start_pos.x + 1,
                y: start_pos.y,
            },
            2 => Point {
                x: start_pos.x,
                y: start_pos.y - 1,
            },
            _ => Point {
                x: start_pos.x,
                y: start_pos.y + 1,
            },
        };
        if Map::in_floor_bounds(stumble_pos) {
            return stumble_pos;
        }
    }
}

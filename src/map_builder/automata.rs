use crate::prelude::*;

use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        const ITERATION_COUNT: i32 = 10;
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: vec![],
            monster_spawns: vec![],
            player_start: None,
            amulet_start: None,
            theme: None,
        };
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..=ITERATION_COUNT {
            self.iteration(&mut mb.map)
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = Some(start);
        mb.amulet_start = Some(mb.find_most_distant());
        mb
    }
}

impl CellularAutomataArchitect {
    pub fn new() -> Box<dyn MapArchitect> {
        Box::new(Self {})
    }

    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        const PERCENT_WALL: i32 = 55;
        let bounds = map.dimensions();
        let idx_to_point = |idx: usize| {
            let w = bounds.x as usize;
            Point::new(idx % w, idx / w)
        };

        map.tiles.iter_mut().enumerate().for_each(|(idx, t)| {
            if !(Map::in_floor_bounds(idx_to_point(idx))) {
                *t = TileType::Wall;
                return;
            }
            let roll = rng.range(0, 100);
            if roll < PERCENT_WALL {
                *t = TileType::Wall;
            } else {
                *t = TileType::Floor;
            }
        })
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(ix, iy)| {
                !(*ix == 0 && *iy == 0) && map.tiles[map_idx(x + *ix, y + *iy)] == TileType::Wall
            })
            .count()
    }

    fn iteration(&mut self, map: &mut Map) {
        const MAX_NEIGHBORS: usize = 4;

        let mut new_tiles = map.tiles.clone();

        for (y, x) in (1..SCREEN_HEIGHT - 1).cartesian_product(1..SCREEN_WIDTH - 1) {
            let neighbors = self.count_neighbors(x, y, map);
            let idx = map_idx(x, y);
            if neighbors > MAX_NEIGHBORS || //.
                  neighbors == 0 || //.
                     !Map::in_floor_bounds(Point::new(x, y))
            {
                new_tiles[idx] = TileType::Wall
            } else {
                new_tiles[idx] = TileType::Floor;
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}

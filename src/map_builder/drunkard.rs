use super::MapArchitect;
use crate::prelude::*;

pub struct DrunkardsWalkArchitect {
    width: i32,
    height: i32,
}

impl DrunkardsWalkArchitect {
    pub fn boxed(width: i32, height: i32) -> Box<dyn MapArchitect> {
        #[cfg(debug_assertions)]
        println!("DrunkardsWalk Architect");
        Box::new(Self { width, height })
    }

    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let stagger_distance = ((self.width * self.height) / 5) as usize;
        for _ in 0..=stagger_distance {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            drunkard_pos = stumble(drunkard_pos, map, rng);
        }
    }

    fn map_is_complete(&self, mb: &MapBuilder) -> bool {
        if mb.amulet_start.is_none() {
            return false;
        }

        let floor_tile_count = mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count();
        let num_tiles = (self.width * self.height) as usize;
        let desired_floor = num_tiles / 3;
        if floor_tile_count < desired_floor {
            return false;
        }

        let amulet_distance = mb.map.distance(
            mb.player_start.expect("No player!"),
            mb.amulet_start.unwrap(),
        );
        let min_amulet_distance = (self.width * self.height) as f32 / 25.0;
        amulet_distance < min_amulet_distance
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        const MAX_DISTANCE_FROM_CENTER: f32 = 2000.0;
        let mut mb = MapBuilder {
            map: Map::new(self.width, self.height),
            rooms: vec![],
            monster_spawns: vec![],
            player_start: None,
            amulet_start: None,
            theme: None,
            width: self.width,
            height: self.height,
        };

        mb.fill(TileType::Wall);
        let center = Point::new(self.width / 2, self.height / 2);
        mb.player_start = Some(center);
        self.drunkard(&center, rng, &mut mb.map);
        while !self.map_is_complete(&mb) {
            self.drunkard(
                &Point::new(rng.range(1, self.width - 1), rng.range(1, self.height - 1)),
                rng,
                &mut mb.map,
            );

            let dijkstra_map = DijkstraMap::new(
                self.width,
                self.height,
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

fn stumble(start_pos: Point, map: &Map, rng: &mut RandomNumberGenerator) -> Point {
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
        if map.in_floor_bounds(stumble_pos) {
            return stumble_pos;
        }
    }
}

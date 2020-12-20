use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: None,
            amulet_start: None,
            theme: None,
        };
        mb.fill(TileType::Floor);
        mb.player_start = Some(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2));
        mb.amulet_start = Some(mb.find_most_distant());
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ))
        }
        mb
    }
}

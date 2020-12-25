use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {
    width: i32,
    height: i32,
}

impl MapArchitect for EmptyArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(self.width, self.height),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: None,
            amulet_start: None,
            theme: None,
            width: self.width,
            height: self.height,
        };
        mb.fill(TileType::Floor);
        mb.player_start = Some(Point::new(self.width / 2, self.height / 2));
        mb.amulet_start = Some(mb.find_most_distant());
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, self.width),
                rng.range(1, self.height),
            ))
        }
        mb
    }
}

#[allow(dead_code)]
impl EmptyArchitect {
    pub fn new(width: i32, height: i32) -> Box<dyn MapArchitect> {
        Box::new(Self { width, height })
    }
}

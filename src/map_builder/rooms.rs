use super::MapArchitect;
use crate::prelude::*;
pub struct RoomsArchitect {
    width: i32,
    height: i32,
}

impl MapArchitect for RoomsArchitect {
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

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = Some(mb.rooms[0].center());
        mb.amulet_start = Some(mb.find_most_distant());
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}

impl RoomsArchitect {
    pub fn new(width: i32, height: i32) -> Box<dyn MapArchitect> {
        Box::new(Self { width, height })
    }
}

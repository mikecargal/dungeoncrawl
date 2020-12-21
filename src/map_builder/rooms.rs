use super::MapArchitect;
use crate::prelude::*;
pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: None,
            amulet_start: None,
            theme: None,
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = Some(mb.rooms[0].center());
        mb.amulet_start = Some(mb.find_most_distant());
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        // display(
        //     "Rooms Map ",
        //     &mb.map,
        //     &mb.player_start.unwrap(),
        //     &mb.amulet_start.unwrap(),
        //     &mb.monster_spawns,
        // );
        mb
    }
}

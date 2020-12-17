use crate::prelude::*;

lazy_static! {
    pub static ref DUNGEON_FLOOR: u16 = to_cp437('.');
    pub static ref DUNGEON_WALL: u16 = to_cp437('#');
    pub static ref FOREST_FLOOR: u16 = to_cp437(';');
    pub static ref FOREST_WALL: u16 = to_cp437('"');
    pub static ref GOBLIN: u16 = to_cp437('g');
    pub static ref ORC: u16 = to_cp437('O');
    pub static ref PLAYER: u16 = to_cp437('@');
    pub static ref AMULET: u16 = to_cp437('|');
}

pub struct DungeonTheme {}


impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> u16 {
        match tile_type {
            TileType::Floor => *DUNGEON_FLOOR,
            TileType::Wall => *DUNGEON_WALL,
        }
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> u16 {
        match tile_type {
            TileType::Floor => *FOREST_FLOOR,
            TileType::Wall => *FOREST_WALL,
        }
    }
}

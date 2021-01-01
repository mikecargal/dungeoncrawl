use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => dungeon_floor_glyph(),
            TileType::Wall => dungeon_wall_glyph(),
        }
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> u16 {
        match tile_type {
            TileType::Floor => forest_floor_glyph(),
            TileType::Wall => forest_wall_glyph(),
        }
    }
}

const DUNGEON_FLOOR_CHAR: char = '.';
const DUNGEON_WALL_CHAR: char = '#';
const FOREST_FLOOR_CHAR: char = ';';
const FOREST_TREE_CHAR: char = '"';
const GOBLIN_CHAR: char = 'g';
const ORC_CHAR: char = 'O';
const PLAYER_CHAR: char = '@';
const AMULET_CHAR: char = '|';

pub fn dungeon_floor_glyph() -> FontCharType {
    to_cp437(DUNGEON_FLOOR_CHAR)
}

pub fn dungeon_wall_glyph() -> FontCharType {
    to_cp437(DUNGEON_WALL_CHAR)
}

pub fn forest_floor_glyph() -> FontCharType {
    to_cp437(FOREST_FLOOR_CHAR)
}

pub fn forest_wall_glyph() -> FontCharType {
    to_cp437(FOREST_TREE_CHAR)
}

pub fn goblin_glyph() -> FontCharType {
    to_cp437(GOBLIN_CHAR)
}

pub fn orc_glyph() -> FontCharType {
    to_cp437(ORC_CHAR)
}

pub fn player_glyph() -> FontCharType {
    to_cp437(PLAYER_CHAR)
}

pub fn amulet_glyph() -> FontCharType {
    to_cp437(AMULET_CHAR)
}

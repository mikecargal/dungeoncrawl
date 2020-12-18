use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
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
    pub fn new() -> Box<dyn MapTheme> {
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

pub fn dungeon_floor_glyph() -> FontCharType {
    to_cp437('.')
}

pub fn dungeon_wall_glyph() -> FontCharType {
    to_cp437('#')
}

pub fn forest_floor_glyph() -> FontCharType {
    to_cp437(';')
}

pub fn forest_wall_glyph() -> FontCharType {
    to_cp437('"')
}

pub fn goblin_glyph() -> FontCharType {
    to_cp437('g')
}

pub fn orc_glyph() -> FontCharType {
    to_cp437('O')
}

pub fn player_glyph() -> FontCharType {
    to_cp437('@')
}

pub fn amulet_glyph() -> FontCharType {
    to_cp437('|')
}
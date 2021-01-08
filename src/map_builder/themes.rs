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
            TileType::Floor => *DUNGEON_FLOOR_GLYPH,
            TileType::Wall => *DUNGEON_WALL_GLYPH,
            TileType::Exit => *STAIRS_GLYPH,
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
            TileType::Floor => *FOREST_FLOOR_GLYPH,
            TileType::Wall => *FOREST_WALL_GLYPH,
            TileType::Exit => *STAIRS_GLYPH,
        }
    }
}

const DUNGEON_FLOOR_CHAR: char = '.';
const DUNGEON_WALL_CHAR: char = '#';
const FOREST_FLOOR_CHAR: char = ';';
const FOREST_TREE_CHAR: char = '"';
const STAIRS_CHAR: char = '>';
const GOBLIN_CHAR: char = 'g';
const ORC_CHAR: char = 'O';
const PLAYER_CHAR: char = '@';
const AMULET_CHAR: char = '|';
const POTION_CHAR: char = '!';
const MAGIC_MAPPER_CHAR: char = '{';

lazy_static! {
    pub static ref DUNGEON_FLOOR_GLYPH: FontCharType = to_cp437(DUNGEON_FLOOR_CHAR);
    pub static ref DUNGEON_WALL_GLYPH: FontCharType = to_cp437(DUNGEON_WALL_CHAR);
    pub static ref FOREST_FLOOR_GLYPH: FontCharType = to_cp437(FOREST_FLOOR_CHAR);
    pub static ref FOREST_WALL_GLYPH: FontCharType = to_cp437(FOREST_TREE_CHAR);
    pub static ref STAIRS_GLYPH: FontCharType = to_cp437(STAIRS_CHAR);
    pub static ref GOBLIN_GLYPH: FontCharType = to_cp437(GOBLIN_CHAR);
    pub static ref ORC_GLYPH: FontCharType = to_cp437(ORC_CHAR);
    pub static ref PLAYER_GLYPH: FontCharType = to_cp437(PLAYER_CHAR);
    pub static ref AMULET_GLYPH: FontCharType = to_cp437(AMULET_CHAR);
    pub static ref POTION_GLYPH: FontCharType = to_cp437(POTION_CHAR);
    pub static ref MAGIC_MAPPER_GLYPH: FontCharType = to_cp437(MAGIC_MAPPER_CHAR);
}

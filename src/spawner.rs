use crate::prelude::*;

const PLAYER_MAX_HEALTH: i32 = 10;
const PLAYER_SIGHT_DISTANCE: i32 = 8;
const MONSTER_SIGHT_DISTANCE: i32 = 6;
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: *PLAYER_GLYPH,
        },
        Health {
            current: PLAYER_MAX_HEALTH,
            max: PLAYER_MAX_HEALTH,
        },
        FieldOfView::new(PLAYER_SIGHT_DISTANCE),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        ChasingPlayer {},
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(MONSTER_SIGHT_DISTANCE),
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    match rng.roll_dice(1, 6) {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    }
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: *POTION_GLYPH,
        },
        Name("Healing Potion"),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: *MAGIC_MAPPER_GLYPH,
        },
        Name("Dungeon Map"),
        ProvidesDungeonMap {},
    ));
}

fn goblin() -> (i32, &'static str, FontCharType) {
    (1, "Goblin", *GOBLIN_GLYPH)
}

fn orc() -> (i32, &'static str, FontCharType) {
    (2, "Orc", *ORC_GLYPH)
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: *AMULET_GLYPH,
        },
        Name("Amulet of Yala"),
    ));
}

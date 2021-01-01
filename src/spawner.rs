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
            glyph: player_glyph(),
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

fn goblin() -> (i32, &'static str, FontCharType) {
    (1, "Goblin", goblin_glyph())
}

fn orc() -> (i32, &'static str, FontCharType) {
    (2, "Orc", orc_glyph())
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: amulet_glyph(),
        },
        Name("Amulet of Yala"),
    ));
}

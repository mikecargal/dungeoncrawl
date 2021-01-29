use crate::prelude::*;

mod template;
use template::Templates;

const PLAYER_MAX_HEALTH: i32 = 10;
const PLAYER_SIGHT_DISTANCE: i32 = 8;
//const MONSTER_SIGHT_DISTANCE: i32 = 6;
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
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
        Damage(1),
    ));
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
        Name(String::from("Amulet of Yala")),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

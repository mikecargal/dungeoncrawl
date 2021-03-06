use crate::prelude::*;

struct FortressStruct<'a> {
    map_str: &'a str,
    x: i32,
    y: i32,
}

const OUTSIDE_FLOOR: char = '-';
const WALL: char = '#';
const MONSTER: char = 'M';
const POSSIBLER_AMULET_POS: char = '.';

const FORTRESS: FortressStruct = FortressStruct {
    map_str: "
------------
---######---
---#....#---
---#.M..#---
-###....###-
--M......M--
-###....###-
---#....#---
---#....#---
---######---
------------
",
    x: 12,
    y: 11,
};

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let amulet_pos = mb
        .amulet_start
        .expect("Can't test placement without an amulet");

    let fortress_vec: Vec<char> = FORTRESS
        .map_str
        .chars()
        .filter(|a| *a != '\r' && *a != '\n')
        .collect();

    let mut amulet_offsets = Vec::new();
    for (_, (y, x)) in (0..FORTRESS.y)
        .cartesian_product(0..FORTRESS.x)
        .enumerate()
        .filter(|(idx, _)| fortress_vec[*idx] == POSSIBLER_AMULET_POS)
    {
        amulet_offsets.push(Point { x, y });
    }

    let valid_positions_around_amulet = amulet_offsets
        .iter()
        .map(|pt| amulet_pos - *pt)
        .filter(|pt| {
            pt.x > 0
                && pt.y > 0
                && pt.x + FORTRESS.x < mb.map.width
                && pt.y + FORTRESS.y < mb.map.height
        })
        .collect::<Vec<Point>>();
    if valid_positions_around_amulet.len() == 0 {
        return;
    }
    let placement =
        valid_positions_around_amulet[rng.range(0, valid_positions_around_amulet.len())];
    let dimensions = Rect::with_size(placement.x, placement.y, FORTRESS.x, FORTRESS.y);
    let points = dimensions.point_set();
    mb.monster_spawns.retain(|pt| !points.contains(pt));

    #[cfg(debug_assertions)]
    println!("Prefab placed at {:?}", &placement);

    for (i, (ty, tx)) in (placement.y..placement.y + FORTRESS.y)
        .cartesian_product(placement.x..placement.x + FORTRESS.x)
        .enumerate()
    {
        let idx = mb.map.index_for(tx, ty);
        match fortress_vec[i] {
            POSSIBLER_AMULET_POS if mb.map.index_to_point2d(idx) == mb.amulet_start.unwrap() => (),
            POSSIBLER_AMULET_POS | OUTSIDE_FLOOR => mb.map.tiles[idx] = TileType::Floor,
            MONSTER => {
                mb.map.tiles[idx] = TileType::Floor;
                mb.monster_spawns.push(Point::new(tx, ty));
            }
            WALL => mb.map.tiles[idx] = TileType::Wall,
            c => println!("No idea what to do with [{}]", c),
        }
    }
}

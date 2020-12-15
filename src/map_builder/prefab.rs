use crate::prelude::*;

struct FortressStruct<'a> {
    map_str: &'a str,
    x: i32,
    y: i32,
}
const FORTRESS: FortressStruct = FortressStruct {
    map_str: "
------------
--########--
--#------#--
--##-M--##--
-###----###-
--M------M--
-###----###-
---#----#---
---##--##---
---######---
------------
",
    x: 12,
    y: 11,
};

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    println!("apply_prefab");
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.x),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.y),
            FORTRESS.x,
            FORTRESS.y,
        );

        let can_place = dimensions.point_set().iter().all(|pt| {
            let idx = mb.map.point2d_to_index(*pt);
            let distance = dijkstra_map.map[idx];
            distance < 2000.0 && distance > 20.0 && *pt == mb.amulet_start
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            println!("before retain {:?}", &mb.monster_spawns);
            mb.monster_spawns.retain(|pt| !points.contains(pt));
            println!("after retain {:?}", &mb.monster_spawns);
        }
        attempts += 1;
    }

    if placement == None {
        println!("could not place prefab");
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .map_str
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        let mut i = 0;
        for ty in placement.y..placement.y + FORTRESS.y {
            for tx in placement.x..placement.x + FORTRESS.x {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{}]", c),
                }
                i += 1;
            }
        }
    }
}

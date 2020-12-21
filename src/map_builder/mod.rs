use crate::prelude::*;
mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

use crate::map_builder::themes::{DungeonTheme, ForestTheme};
use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use prefab::apply_prefab;
use rooms::RoomsArchitect;
use std::cmp::{max, min};
pub use themes::*;

const NUM_ROOMS: usize = (SCREEN_WIDTH * SCREEN_HEIGHT / 100) as usize; //20;

const UNREACHABLE: f32 = std::f32::MAX;

pub trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Option<Point>,
    pub amulet_start: Option<Point>,
    pub theme: Option<Box<dyn MapTheme>>,
}

lazy_static! {
    static ref ARCHICTECT_CREATORS: Vec<fn() -> Box<dyn MapArchitect>> = vec![
        || {
            #[cfg(debug_assertions)]
            println!("Rooms Architect");
            RoomsArchitect::new()
        },
        || {
            #[cfg(debug_assertions)]
            println!("DrunkardsWalk Architect");
            DrunkardsWalkArchitect::new()
        },
        || {
            #[cfg(debug_assertions)]
            println!("Cellular Automata Architect");
            CellularAutomataArchitect::new()
        },
    ];
}

lazy_static! {
    static ref THEME_CREATORS: Vec<fn() -> Box<dyn MapTheme>> =
        vec![|| DungeonTheme::new(), || ForestTheme::new(),];
}

fn get_random_from<T>(creators: &Vec<fn() -> T>, rng: &mut RandomNumberGenerator) -> T {
    creators[rng.range(0, creators.len())]()
}

impl MapBuilder {
    pub fn build(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = get_random_from(&ARCHICTECT_CREATORS, rng).build(rng);
        // let mut mb = ARCHICTECT_CREATORS[2]().build(rng);
        apply_prefab(&mut mb, rng);
        mb.theme = Some(get_random_from(&THEME_CREATORS, rng));
        #[cfg(debug_assertions)]
        {
            display(
                "Map ",
                &mb.map,
                &mb.player_start.unwrap(),
                &mb.amulet_start.unwrap(),
                &mb.monster_spawns,
            );
            println!("Amulet is at {:?}", mb.amulet_start);
            println!(
                "amulet is {} steps from player",
                mb.map
                    .distance(mb.player_start.unwrap(), mb.amulet_start.unwrap())
            )
        }
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(
                self.player_start
                    .expect("Can't find distance from non-existent player"),
            )],
            &self.map,
            DISTANCE_MAX_DEPTH,
        );

        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist != &UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        const ROOM_MIN_DIMENSION: i32 = 2;
        const ROOM_MAX_WIDTH: i32 = 10;
        const ROOM_MAX_HEIGHT: i32 = SCREEN_WIDTH * ROOM_MAX_WIDTH / SCREEN_HEIGHT;
        const MAX_ATTEMPTS: i32 = 1_000;
        let mut attempts = 0;
        while self.rooms.len() < NUM_ROOMS && attempts < MAX_ATTEMPTS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - ROOM_MAX_WIDTH),
                rng.range(1, SCREEN_HEIGHT - ROOM_MAX_HEIGHT),
                rng.range(ROOM_MIN_DIMENSION, ROOM_MAX_WIDTH + 1),
                rng.range(ROOM_MIN_DIMENSION, ROOM_MAX_HEIGHT + 1),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH //.
                        && p.y > 0 && p.y < SCREEN_HEIGHT
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
            attempts += 1;
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| {
            let ac = a.center();
            let bc = b.center();
            let av = ac.x + ac.y;
            let bv = bc.x + bc.y;
            av.cmp(&bv)
        });
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if fifty_fifty(rng) {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = (SCREEN_HEIGHT * SCREEN_HEIGHT / 40) as usize;
        const MIN_MONSTER_DISTANCE: f32 = 10.0;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        >= MIN_MONSTER_DISTANCE
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index.clone()]);
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
}

pub fn display(
    title: &str,
    map: &Map,
    player_start: &Point,
    amulet_start: &Point,
    monster_spawns: &[Point],
) {
    //----- Display Chars
    const FLOOR: char = '.';
    const WALL: char = '#';
    const PLAYER: char = '@';
    const AMULET: char = 'A';
    const MONSTER: char = 'M';

    use colored::*;
    let mut output = vec!['.'; NUM_TILES];

    map.tiles.iter().enumerate().for_each(|(idx, t)| match *t {
        TileType::Floor => output[idx] = FLOOR,
        TileType::Wall => output[idx] = WALL,
    });

    output[map.point2d_to_index(*player_start)] = PLAYER;
    output[map.point2d_to_index(*amulet_start)] = AMULET;
    monster_spawns.iter().for_each(|p| {
        output[map.point2d_to_index(*p)] = MONSTER;
    });

    println!(
        "----------------------\n{}\n----------------------",
        title.bright_yellow()
    );
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            match output[map_idx(x, y)] {
                WALL => print!("{}", WALL.to_string().bright_green()),
                PLAYER => print!("{}", PLAYER.to_string().bright_yellow()),
                MONSTER => print!("{}", MONSTER.to_string().bright_red()),
                AMULET => print!("{}", AMULET.to_string().bright_magenta()),
                _ => print!("{}", ".".truecolor(64, 64, 64)),
            }
        }
        println!();
    }
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

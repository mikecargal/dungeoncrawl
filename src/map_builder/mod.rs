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
use std::{
    cmp::{max, min, Ordering},
    collections::HashMap,
};
pub use themes::*;

const TILES_TO_ROOM_RATIO: usize = 200;
const MIN_ROOMS: usize = 10;

const UNREACHABLE: f32 = std::f32::MAX;

pub trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub width: i32,
    pub height: i32,
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Option<Point>,
    pub amulet_start: Option<Point>,
    pub theme: Option<Box<dyn MapTheme>>,
}

type ArchitectCreator = fn(width: i32, height: i32) -> Box<dyn MapArchitect>;
const ROOMS_CREATOR: ArchitectCreator = RoomsArchitect::boxed;
const DRUNKARDS_WALK_CREATOR: ArchitectCreator = DrunkardsWalkArchitect::boxed;
const CELLULAR_AUTOMATA_CREATOR: ArchitectCreator = CellularAutomataArchitect::boxed;
const ARCHICTECT_CREATORS: &[ArchitectCreator] = &[
    ROOMS_CREATOR,
    DRUNKARDS_WALK_CREATOR,
    CELLULAR_AUTOMATA_CREATOR,
];

type ThemeCreator = fn() -> Box<dyn MapTheme>;
const DUNGEON_THEME_CREATOR: ThemeCreator = DungeonTheme::boxed;
const FOREST_THEME_CREATOR: ThemeCreator = ForestTheme::boxed;
const THEME_CREATORS: &[ThemeCreator] = &[DUNGEON_THEME_CREATOR, FOREST_THEME_CREATOR];

fn get_random_architect(
    creators: &[ArchitectCreator],
    width: i32,
    height: i32,
    rng: &mut RandomNumberGenerator,
) -> Box<dyn MapArchitect> {
    creators[rng.range(0, creators.len())](width, height)
}

fn get_random_theme(
    creators: &[ThemeCreator],
    rng: &mut RandomNumberGenerator,
) -> Box<dyn MapTheme> {
    creators[rng.range(0, creators.len())]()
}

impl MapBuilder {
    pub fn build(config: &Config, rng: &mut RandomNumberGenerator) -> Self {
        let WorldDimensions {
            world_width: width,
            world_height: height,
            ..
        } = config.world_dimensions;
        let mut mb = match config.architect {
            ArchitectChoice::Random => {
                get_random_architect(ARCHICTECT_CREATORS, width, height, rng)
            }
            ArchitectChoice::Rooms => ROOMS_CREATOR(width, height),
            ArchitectChoice::CellularAutomata => CELLULAR_AUTOMATA_CREATOR(width, height),
            ArchitectChoice::Drunkard => DRUNKARDS_WALK_CREATOR(width, height),
        }
        .build(rng);
        apply_prefab(&mut mb, rng);
        mb.theme = Some(match config.theme {
            ThemeChoice::Dungeon => DUNGEON_THEME_CREATOR(),
            ThemeChoice::Forest => FOREST_THEME_CREATOR(),
            ThemeChoice::Random => get_random_theme(THEME_CREATORS, rng),
        });
        #[cfg(debug_assertions)]
        {
            display(
                "Map ",
                &mb.map,
                &mb.player_start,
                &mb.amulet_start,
                &mb.monster_spawns,
                &None,
                &None,
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
            self.width,
            self.height,
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
        let room_max_width: i32 = (self.width / 5).min(10);
        let room_max_height: i32 = (self.height / 5).min(10);
        const MAX_ATTEMPTS: i32 = 1_000;
        let mut attempts = 0;
        let num_rooms = (self.width * self.height) as usize
            / rng
                .range(TILES_TO_ROOM_RATIO, TILES_TO_ROOM_RATIO * 2)
                .min(MIN_ROOMS);
        println!(
            "attempting {} rooms max=({}x{})",
            num_rooms, room_max_width, room_max_height
        );
        while self.rooms.len() < num_rooms && attempts < MAX_ATTEMPTS {
            let room = Rect::with_size(
                rng.range(1, (self.width - 1) - room_max_width),
                rng.range(1, (self.height - 1) - room_max_height),
                rng.range(ROOM_MIN_DIMENSION, room_max_width + 1),
                rng.range(ROOM_MIN_DIMENSION, room_max_height + 1),
            );

            let valid_placement = !self.rooms.iter().any(|r| r.intersect(&room));

            if valid_placement {
                assert!(room.y2 < self.height);
                room.for_each(|p| {
                    if p.x > 0 && p.x < self.width //.
                        && p.y > 0 && p.y < self.height
                    {
                        let idx = self.map.point2d_to_index(p);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
            attempts += 1;
        }
        println!("actual rooms = {}", self.rooms.len());
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
        #[cfg(debug_assertions)]
        {
            println!("Start");
            display(
                "Map ",
                &self.map,
                &Some(rooms.first().unwrap().center()),
                &None,
                &[],
                &None,
                &None,
            );
        }

        let mut map_idx_to_room: HashMap<usize, &Rect> = HashMap::new();
        rooms.iter().for_each(|room| {
            let Point { x, y } = room.center();
            map_idx_to_room.insert(self.map.index_for(x, y), room);
        });

        loop {
            let dijkstra_map = DijkstraMap::new(
                self.width,
                self.height,
                &vec![self.map.point2d_to_index(rooms.first().unwrap().center())],
                &self.map,
                DISTANCE_MAX_DEPTH,
            );

            let (reachable_rooms, unreachable_rooms): (Vec<&Rect>, Vec<&Rect>) = dijkstra_map
                .map
                .iter()
                .enumerate()
                .map(|(idx, distance)| (distance, map_idx_to_room.get(&idx)))
                .filter(|(_, room)| room.is_some())
                .partition_map(|(distance, room)| {
                    if *distance != UNREACHABLE {
                        Either::Left(*room.unwrap())
                    } else {
                        Either::Right(*room.unwrap())
                    }
                });

            if unreachable_rooms.is_empty() {
                break;
            }
            let shortest_tunnel = reachable_rooms
                .iter()
                .cartesian_product(unreachable_rooms.iter())
                .map(|(room1, room2)| tunnel_between(room1, room2))
                .min()
                .unwrap();

            let Tunnel { start, end, .. } = shortest_tunnel;

            if fifty_fifty(rng) {
                self.apply_horizontal_tunnel(start.x, end.x, start.y);
                self.apply_vertical_tunnel(start.y, end.y, end.x);
            } else {
                self.apply_vertical_tunnel(start.y, end.y, start.x);
                self.apply_horizontal_tunnel(start.x, end.x, end.y);
            }
            #[cfg(debug_assertions)]
            {
                println!("tunnel from {:?} to {:?}", start, end);
                display(
                    "Map ",
                    &self.map,
                    &None,
                    &None,
                    &[],
                    &Some(start),
                    &Some(end),
                );
            }
        }
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let num_monsters = (self.width * self.height / 40) as usize;
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
        for _ in 0..num_monsters {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index.clone()]);
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
}

#[derive(Debug, Eq)]
struct Tunnel {
    pub start: Point,
    pub end: Point,
    pub length: i32,
}

impl Ord for Tunnel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length.cmp(&other.length)
    }
}

impl PartialOrd for Tunnel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tunnel {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
    }
}

fn tunnel_between(room1: &Rect, room2: &Rect) -> Tunnel {
    assert!(!room1.intersect(room2));
    let (r1x, r2x) = match (room1, room2) {
        (Rect { x2, .. }, Rect { x1, .. }) if x2 < x1 => (*x2, *x1),
        (Rect { x1, .. }, Rect { x2, .. }) if x1 > x2 => (*x1, *x2),
        (r1, r2) => {
            let (overlap_x1, overlap_x2) = (r1.x1.max(r2.x1), r1.x2.min(r2.x2));
            let median = overlap_x1 + (overlap_x2 - overlap_x1) / 2;
            (median, median)
        }
    };
    let (r1y, r2y) = match (room1, room2) {
        (Rect { y2, .. }, Rect { y1, .. }) if y2 < y1 => (*y2, *y1),
        (Rect { y1, .. }, Rect { y2, .. }) if y1 > y2 => (*y1, *y2),
        (r1, r2) => {
            let (overlap_y1, overlap_y2) = (r1.y1.max(r2.y1), r1.y2.min(r2.y2));
            let median = overlap_y1 + (overlap_y2 - overlap_y1) / 2;
            (median, median)
        }
    };
    let tunnel_distance = (r1x - r2x).abs() + (r1y - r2y).abs();
    Tunnel {
        start: Point::new(r1x, r1y),
        end: Point::new(r2x, r2y),
        length: tunnel_distance,
    }
}

pub fn display(
    title: &str,
    map: &Map,
    player_start: &Option<Point>,
    amulet_start: &Option<Point>,
    monster_spawns: &[Point],
    start: &Option<Point>,
    end: &Option<Point>,
) {
    //----- Display Chars
    const FLOOR: char = '.';
    const WALL: char = '#';
    const PLAYER: char = '@';
    const AMULET: char = 'A';
    const MONSTER: char = 'M';
    const START: char = 'S';
    const END: char = 'E';

    use colored::*;
    let mut output = vec!['.'; (map.width * map.height) as usize];

    map.tiles.iter().enumerate().for_each(|(idx, t)| match *t {
        TileType::Floor => output[idx] = FLOOR,
        TileType::Wall => output[idx] = WALL,
    });

    if let Some(pos) = player_start {
        output[map.point2d_to_index(*pos)] = PLAYER;
    }
    if let Some(pos) = amulet_start {
        output[map.point2d_to_index(*pos)] = AMULET;
    }
    if let Some(pos) = start {
        output[map.point2d_to_index(*pos)] = START;
    }
    if let Some(pos) = end {
        output[map.point2d_to_index(*pos)] = END;
    }
    monster_spawns.iter().for_each(|p| {
        output[map.point2d_to_index(*p)] = MONSTER;
    });

    println!(
        "----------------------\n{}\n----------------------",
        title.bright_yellow()
    );
    for y in 0..map.height {
        for x in 0..map.width {
            match output[map.index_for(x, y)] {
                WALL => print!("{}", WALL.to_string().bright_green()),
                PLAYER => print!("{}", PLAYER.to_string().bright_yellow()),
                MONSTER => print!("{}", MONSTER.to_string().bright_red()),
                AMULET => print!("{}", AMULET.to_string().bright_magenta()),
                START => print!("{}", START.to_string().bright_yellow()),
                END => print!("{}", END.to_string().bright_yellow()),
                _ => print!("{}", ".".truecolor(64, 64, 64)),
            }
        }
        println!();
    }
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

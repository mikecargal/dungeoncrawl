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

const NUM_ROOMS: usize = 20;

const UNREACHABLE: f32 = std::f32::MAX;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Option<Box<dyn MapTheme>>,
}

impl MapBuilder {
    pub fn build(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => {
                println!("Rooms Architect");
                Box::new(RoomsArchitect {})
            }
            1 => {
                println!("Drunkard's walk Architect");
                Box::new(DrunkardsWalkArchitect {})
            }
            _ => {
                println!("Cellular Automata Architect");
                Box::new(CellularAutomataArchitect {})
            }
        };
        let mut mb = architect.build(rng);
        apply_prefab(&mut mb, rng);

        mb.theme = match rng.range(0, 2) {
            0 => Some(DungeonTheme::new()),
            _ => Some(ForestTheme::new()),
        };

        println!("Amulet is at {:?}", mb.amulet_start);
        println!(
            "monster.spawns[{}]={:?}",
            &mb.monster_spawns.len(),
            &mb.monster_spawns
        );
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
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
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
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
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
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
    use colored::*;
    let mut output = vec!['.'; NUM_TILES];

    map.tiles.iter().enumerate().for_each(|(idx, t)| match *t {
        TileType::Floor => output[idx] = '.',
        TileType::Wall => output[idx] = '#',
    });

    output[map.point2d_to_index(*player_start)] = '@';
    output[map.point2d_to_index(*amulet_start)] = 'A';
    monster_spawns.iter().for_each(|p| {
        output[map.point2d_to_index(*p)] = 'M';
    });

    //print!("\x1B[2J");
    println!(
        "----------------------\n{}\n----------------------",
        title.bright_yellow()
    );
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            match output[map_idx(x, y)] {
                '#' => print!("{}", "#".bright_green()),
                '@' => print!("{}", "@".bright_yellow()),
                'M' => print!("{}", "M".bright_red()),
                'A' => print!("{}", "A".bright_magenta()),
                _ => print!("{}", ".".truecolor(64, 64, 64)),
            }
        }
        println!();
    }

    //       let mut ignore_me = String::new();
    //       stdin()
    //           .read_line(&mut ignore_me)
    //           .expect("Failed to read line");
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn in_floor_bounds(point: Point) -> bool {
        point.x >= 1 && //.
            point.x < SCREEN_WIDTH-1 && //.
            point.y >= 1 && //.
            point.y < SCREEN_HEIGHT-1
    }

    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && //.
            point.x < SCREEN_WIDTH && //.
            point.y >= 0 && //.
            point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(map_idx(point.x, point.y))
        } else {
            None
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                Some(self.point2d_to_index(destination))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn distance(&self, pt_a: Point, pt_b: Point) -> f32 {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.point2d_to_index(pt_a)],
            self,
            DISTANCE_MAX_DEPTH,
        );
        let pt_b_idx = self.point2d_to_index(pt_b);
        *dijkstra_map
            .map
            .iter()
            .enumerate()
            .find(|(idx, _)| *idx == pt_b_idx)
            .unwrap()
            .1
    }
}
lazy_static! {
    static ref LEFT: Point = Point::new(-1, 0);
    static ref RIGHT: Point = Point::new(1, 0);
    static ref UP: Point = Point::new(0, -1);
    static ref DOWN: Point = Point::new(0, 1);
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, *LEFT) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, *RIGHT) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, *UP) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, *DOWN) {
            exits.push((idx, 1.0))
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1), //.
            self.index_to_point2d(idx2),
        )
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

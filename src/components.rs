pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player {
    pub map_level: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingRandomly;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Item;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AmuletOfYala;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldOfView {
    pub visible_tiles: Option<HashSet<Point>>,
    pub radius: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap {}

#[derive(Clone, PartialEq)]
pub struct Carried {
    pub by: Entity,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: None,
            radius,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: None,
            radius: self.radius,
        }
    }

    pub fn is_visible(&self, pt: &Point) -> bool {
        match &self.visible_tiles {
            Some(vt) => vt.contains(pt),
            None => false,
        }
    }
}

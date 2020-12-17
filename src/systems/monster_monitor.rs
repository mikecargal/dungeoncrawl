use crate::prelude::*;

#[cfg(debug_assertions)]
use std::collections::HashSet;

#[system]
#[read_component(Point)]
#[read_component(Enemy)]
#[cfg(debug_assertions)]
pub fn monster_monitor(ecs: &mut SubWorld) {
    let mut set = HashSet::new();
    <&Point>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .for_each(|point| {
            if set.contains(point) {
                println!("More than one monster @{:?}", point);
            }
            set.insert(point);
        });
}

#[system]
#[cfg(not(debug_assertions))]
pub fn monster_monitor() {}

use crate::prelude::*;
use std::collections::HashSet;

#[system]
#[read_component(Point)]
#[read_component(Enemy)]
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

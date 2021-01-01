use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Enemy)]
#[cfg(debug_assertions)]
pub fn monster_monitor(ecs: &mut SubWorld) {
    <&Point>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .combinations(2)
        .for_each(|points| {
            if points[0] == points[1] {
                println!("More than one monster @{:?}", points[0]);
            }
        });
}

#[system]
#[cfg(not(debug_assertions))]
pub fn monster_monitor() {}

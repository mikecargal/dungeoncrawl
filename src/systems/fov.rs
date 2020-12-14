use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut views = <(&Point, &mut FieldOfView)>::query();
    views
        .iter_mut(ecs)
        .filter(|(_, fov)| fov.visible_tiles == None)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = Some(field_of_view_set(*pos, fov.radius, map));
            //  fov.is_dirty = false;
            // if pos.y == 1 {
            //     let interesting_fov: Vec<&Point> = fov
            //       .visible_tiles
            //     .as_ref()
            //   .unwrap()
            //  .iter()
            //  .filter(|pt| pt.y < 2 || pt.x < 2)
            //  .collect();
            //  println!("FOV @{:?}={:?}", pos, fov.visible_tiles);
            // }
        })
}

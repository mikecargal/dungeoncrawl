use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    draw_batch.target(ENTITY_LAYER.id);
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });
    draw_batch
        .submit(ENTITY_LAYER.z_order)
        .expect("Entity Batch Error");
}

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER.id);
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| {
            let visible_to_player = player_fov.is_visible(pos);
            **pos == map_pos && visible_to_player
        })
        .for_each(|(entity, _, name)| {
            let screen_pos = (*mouse_pos * (GAME_TILE_WIDTH / HUD_TILE_WIDTH)) - 1;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos, &display);
            draw_batch
                .submit(ENTITY_LAYER.z_order + 100)
                .expect("Batch Error on tooltip");
        });
}

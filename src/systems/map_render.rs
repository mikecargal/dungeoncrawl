use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(BACKGROUND_LAYER.id);
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let visible_to_player = player_fov.is_visible(&pt);
            let idx = map_idx(x, y);
            if map.in_bounds(pt) && (visible_to_player | map.revealed_tiles[idx]) {
                let tint = if player_fov.is_visible(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch
        .submit(BACKGROUND_LAYER.z_order)
        .expect("Map Batch Error");
}

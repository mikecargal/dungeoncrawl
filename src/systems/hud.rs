use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER.id);
    draw_batch.print_centered(0, "Explore the Dungeon.  Cursor keys to move.");
    let health_x = (SCREEN_HEIGHT - 1) * 2;
    draw_batch.bar_horizontal(
        Point::new(0, health_x),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        health_x,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.submit(HUD_LAYER.z_order).expect("Batch error");
}

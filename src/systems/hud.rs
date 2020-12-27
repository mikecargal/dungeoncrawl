use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld, #[resource] map: &Map) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER.id);
    draw_batch.print_centered(0, "Explore the Dungeon.  Cursor keys to move.");
    let health_x = (map.height - 1) * 2;
    let health_color = match player_health.current {
        h if h > 7 => GREEN,
        h if h > 3 => YELLOW,
        _ => RED,
    };
    draw_batch.bar_horizontal(
        Point::new(0, health_x),
        map.width * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(health_color, BLACK),
    );
    draw_batch.print_color_centered(
        health_x,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, health_color),
    );
    draw_batch.submit(HUD_LAYER.z_order).expect("Batch error");
}

use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld, #[resource] map: &Map) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER.id);
    draw_batch.print_centered(0, "Explore the Dungeon.  Cursor keys to move.");
    let health_x = (map.height - 1) * 2;
    let health_color = match player_health.current {
        10 => WHITE,
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

    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;
    item_query
        .iter(ecs)
        .filter(|(_item, _name, carried)| carried.by == player)
        .for_each(|(_item, name, _carried)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }
    draw_batch.submit(HUD_LAYER.z_order).expect("Batch error");
}

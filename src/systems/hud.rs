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

    let (_player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new(map.width * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let item_top_line = 2;
    let item_column = 3;
    let mut y = item_top_line;
    item_query
        .iter(ecs)
        .filter(|(_item, _name, carried)| carried.by == player)
        .for_each(|(_item, name, _carried)| {
            y += 1;
            draw_batch.print(
                Point::new(item_column, y),
                format!("{} : {}", y - 2, &name.0),
            );
        });
    if y > item_top_line {
        draw_batch.print_color(
            Point::new(item_column, item_top_line),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }
    draw_batch.submit(HUD_LAYER.z_order).expect("Batch error");
}

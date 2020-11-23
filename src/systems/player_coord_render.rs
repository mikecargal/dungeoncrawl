use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn player_coord_render(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| {
            let posn = format!("{}:{}", pos.x, pos.y);
            for (i, c) in posn.chars().enumerate() {
                draw_batch.set(
                    Point::new(i, 0),
                    ColorPair::new(RGB::named(GREEN), RGB::named(BLACK)),
                    to_cp437(c),
                );
            }
        });
    draw_batch.submit(15000).expect("Coord Batch Error");
}
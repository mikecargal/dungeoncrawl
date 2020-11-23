use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn player_coord_render(ecs: &SubWorld /*, #[resource] camera: &Camera*/) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    //let offset = Point::new(camera.left_x, camera.top_y);
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| {
            let posn = format!("{}:{}", pos.x, pos.y);
            for (i, c) in posn.chars().enumerate() {
                // println!("{}-{}", i, c);
                draw_batch.set(
                    Point::new(i, 0),
                    ColorPair::new(RGB::named(GREEN), RGB::named(BLACK)),
                    to_cp437(c),
                );
            }
            // println!("pos: {}", posn);

            //draw_batch.set(*pos - offset, render.color, render.glyph);
        });
    draw_batch.submit(15000).expect("Coord Batch Error");
}

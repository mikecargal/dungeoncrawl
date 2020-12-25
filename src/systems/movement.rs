use crate::prelude::*;

#[system(for_each)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        if !<&Point>::query()
            .filter(component::<Enemy>())
            .iter(ecs)
            .any(|pt| *pt == want_move.destination)
        {
            commands.add_component(want_move.entity, want_move.destination);
            if let Ok(entry) = ecs.entry_ref(want_move.entity) {
                if let Ok(fov) = entry.get_component::<FieldOfView>() {
                    commands.add_component(want_move.entity, fov.clone_dirty());
                    if entry.get_component::<Player>().is_ok() {
                        camera.on_player_move(want_move.destination);
                        if let Some(vt) = &fov.visible_tiles.as_ref() {
                            vt.iter().for_each(|pos| {
                                map.revealed_tiles
                                    [Map::index_for_x_y_width(pos.x, pos.y, map.width)] = true;
                            })
                        }
                    }
                }
            }
        }
    }
    commands.remove(*entity);
}

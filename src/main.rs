#![warn(clippy::pedantic)]

mod camera;
mod components;
mod config;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::config::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use itertools::*;
    pub use lazy_static::*;
    pub use legion::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;

    pub const GAME_TILE_WIDTH: i32 = 32;
    pub const GAME_TILE_HEIGHT: i32 = 32;
    pub const HUD_TILE_WIDTH: i32 = 8;
    pub const HUD_TILE_HEIGHT: i32 = 8;

    #[derive(Debug, Copy, Clone)]
    pub struct LayerDef {
        pub id: usize,
        pub z_order: usize,
    }

    const BACKGROUND_LAYER_ID: usize = 0;
    const BACKGROUND_LAYER_Z_ORDER: usize = 0;

    const ENTITY_LAYER_ID: usize = 1;
    const ENTITY_LAYER_Z_ORDER: usize = BACKGROUND_LAYER_Z_ORDER + 5_000;

    const HUD_ID: usize = 2;
    const HUD_LAYER_Z_ORDER: usize = ENTITY_LAYER_Z_ORDER + 5_000;

    pub const BACKGROUND_LAYER: LayerDef = LayerDef {
        id: BACKGROUND_LAYER_ID,
        z_order: BACKGROUND_LAYER_Z_ORDER,
    };
    pub const ENTITY_LAYER: LayerDef = LayerDef {
        id: ENTITY_LAYER_ID,
        z_order: ENTITY_LAYER_Z_ORDER,
    };
    pub const HUD_LAYER: LayerDef = LayerDef {
        id: HUD_ID,
        z_order: HUD_LAYER_Z_ORDER,
    };

    pub const DISTANCE_MAX_DEPTH: f32 = 1024.0;
    pub const RENDER_LAYERS: [LayerDef; 3] = [BACKGROUND_LAYER, ENTITY_LAYER, HUD_LAYER];

    pub fn fifty_fifty(rng: &mut RandomNumberGenerator) -> bool {
        rng.range(0, 2) == 1
    }
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    config: Config,
}

struct NewGameData {
    ecs: World,
    resources: Resources,
}

impl State {
    fn new(config: Config) -> Self {
        let NewGameData { ecs, resources } = State::new_game_data(&config);
        Self {
            ecs,
            resources,
            input_systems: build_input_schedule(),
            player_systems: build_player_schedule(),
            monster_systems: build_monster_schedule(),
            config,
        }
    }

    fn new_game_data(config: &Config) -> NewGameData {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::build(&config, &mut rng);
        spawn_player(
            &mut ecs,
            map_builder.player_start.expect("What?? No player?"),
        );

        let exit_idx = map_builder
            .map
            .point2d_to_index(map_builder.amulet_start.expect("There was no Amulet!!"));
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut ecs, &mut rng, *pos));
        resources.insert(map_builder.map);
        let WorldDimensions {
            display_width,
            display_height,
            ..
        } = config.world_dimensions;
        resources.insert(Camera::new(
            map_builder.player_start.unwrap(),
            display_width,
            display_height,
        ));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(rng);
        resources.insert(map_builder.theme);
        NewGameData { ecs, resources }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER.id);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of YALA remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER.id);
        ctx.print_color_centered(2, GREEN, BLACK, "Your have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins...",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        let NewGameData { ecs, resources } = Self::new_game_data(&self.config);
        self.ecs = ecs;
        self.resources = resources;
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_entity, carry)| carry.by == player_entity)
            .map(|(entity, _carry)| *entity)
            .for_each(|entity| {
                entities_to_keep.insert(entity);
            });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for entity in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(entity) {
                cb.remove(*entity);
            }
        }
        cb.flush(&mut self.ecs);
        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.visible_tiles = None);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::build(&self.config, &mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                let ps = map_builder.player_start.unwrap();
                pos.x = ps.x;
                pos.y = ps.y;
            });

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start.unwrap());
        } else {
            let exit_idx = map_builder
                .map
                .point2d_to_index(map_builder.amulet_start.unwrap());
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(
            map_builder.player_start.unwrap(),
            self.config.world_dimensions.display_width,
            self.config.world_dimensions.display_height,
        ));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for layer in RENDER_LAYERS.iter() {
            ctx.set_active_console((*layer).id);
            ctx.cls();
        }
        self.resources.insert(ctx.key);
        ctx.set_active_console(ENTITY_LAYER.id);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }
        render_draw_buffer(ctx).expect("Render Error");
    }
}

fn main() -> BError {
    let config = config::parse_command_line_args();
    let WorldDimensions {
        display_width,
        display_height,
        ..
    } = config.world_dimensions;
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_platform_specific(30.0)
        .with_dimensions(display_width, display_height)
        .with_tile_dimensions(GAME_TILE_WIDTH, GAME_TILE_WIDTH)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", GAME_TILE_WIDTH, GAME_TILE_WIDTH)
        .with_font("terminal8x8.png", HUD_TILE_WIDTH, HUD_TILE_WIDTH)
        .with_simple_console(display_width, display_height, "dungeonfont.png")
        .with_simple_console_no_bg(display_width, display_height, "dungeonfont.png")
        .with_simple_console_no_bg(
            display_width * (GAME_TILE_WIDTH / HUD_TILE_WIDTH),
            display_height * (GAME_TILE_HEIGHT / HUD_TILE_HEIGHT),
            "terminal8x8.png",
        )
        .build()?;

    main_loop(context, State::new(config))
}

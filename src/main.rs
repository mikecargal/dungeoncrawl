#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use lazy_static::*;
    pub use legion::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
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
}

struct NewGameData {
    ecs: World,
    resources: Resources,
}

impl State {
    fn new() -> Self {
        let NewGameData { ecs, resources } = State::new_game_data();
        Self {
            ecs,
            resources,
            input_systems: build_input_schedule(),
            player_systems: build_player_schedule(),
            monster_systems: build_monster_schedule(),
        }
    }

    fn new_game_data() -> NewGameData {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);
        spawn_player(
            &mut ecs,
            map_builder.player_start.expect("What?? No player?"),
        );
        spawn_amulet_of_yala(
            &mut ecs,
            map_builder.amulet_start.expect("There was no Amulet!!"),
        );
        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, *pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start.unwrap()));
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
            "Slain by a monster, your her's journey has come to a premature end.",
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
            "You put on the Amulet of Yala and feel its power course through your veins..",
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
        let NewGameData { ecs, resources } = State::new_game_data();
        self.ecs = ecs;
        self.resources = resources;
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
        }
        render_draw_buffer(ctx).expect("Render Error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(GAME_TILE_WIDTH, GAME_TILE_WIDTH)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", GAME_TILE_WIDTH, GAME_TILE_WIDTH)
        .with_font("terminal8x8.png", HUD_TILE_WIDTH, HUD_TILE_WIDTH)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(
            DISPLAY_WIDTH * (GAME_TILE_WIDTH / HUD_TILE_WIDTH),
            DISPLAY_HEIGHT * (GAME_TILE_HEIGHT / HUD_TILE_HEIGHT),
            "terminal8x8.png",
        )
        .build()?;

    main_loop(context, State::new())
}

use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;

// Consts
const BKG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PLAYER_SPRITE: &str = "player_b_01.png";
const PLAYER2_SPRITE: &str = "player2_b_01.png";
const TRAIL_SPRITE: &str = "laser_a_01.png";
const TRAIL2_SPRITE: &str = "laser_b_01.png";
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 100.;
const ROTATION_SPEED: f32 = 1.;

struct GameTextures {
    player: Handle<Image>,
    player2: Handle<Image>,
    trail: Handle<Image>,
    trail2: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BKG_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Tronie".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // start camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add game textures
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player2: asset_server.load(PLAYER2_SPRITE),
        trail: asset_server.load(TRAIL_SPRITE),
        trail2: asset_server.load(TRAIL2_SPRITE),
    };
    commands.insert_resource(game_textures);
}

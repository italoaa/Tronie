#![allow(unused)]
use bevy::prelude::*;
use components::*;
use player::PlayerPlugin;

mod components;
mod player;

// Consts
const BKG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PLAYER_SPRITE: &str = "player_b_01.png";
const PLAYER_SIZE: (f32, f32) = (98., 75.);

struct GameTextures {
    player: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BKG_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Test".to_string(),
            width: 500.0,
            height: 500.0,
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
    };
    commands.insert_resource(game_textures);
}

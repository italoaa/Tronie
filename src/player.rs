use crate::{GameTextures, PLAYER_SIZE};
use bevy::prelude::*;

use crate::components::{Player, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system);
    }
}

fn spawn_player_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // start player
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity { x: 1., y: 1. });
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, &mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
    }
}

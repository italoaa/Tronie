use crate::{
    components::{Collider, RotationalVelocity, Trail},
    GameTextures, BASE_SPEED, ROTATION_SPEED, TIME_STEP,
};
use bevy::{app::AppExit, prelude::*, sprite::collide_aabb::collide};

use crate::components::{Player, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_system_to_stage(CoreStage::PostUpdate, player_collition_system)
            .add_system_to_stage(CoreStage::PreUpdate, spawn_player_trail_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system)
            .add_system(despawn_player_trail_system);
    }
}

fn spawn_player_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // start player
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(-100., 0., 10.),
                scale: Vec3::new(0.2, 0.2, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { id: 1 })
        .insert(Velocity { velocity: 0. })
        .insert(RotationalVelocity { omega: 0. });

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player2.clone(),
            transform: Transform {
                translation: Vec3::new(100., 0., 10.),
                scale: Vec3::new(0.2, 0.2, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { id: 2 })
        .insert(Velocity { velocity: 0. })
        .insert(RotationalVelocity { omega: 0. });
}

fn spawn_player_trail_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(&Transform, &Player)>,
    time: Res<Time>,
) {
    let mut player_pos: Vec3;
    let mut player_rot: Quat;
    let mut player_sprite: Handle<Image>;
    if time.seconds_since_startup() > 3. {
        for (transform, player) in query.iter() {
            player_pos = transform.translation;
            player_rot = transform.rotation;
            // trail sprite
            if player.id == 1 {
                player_sprite = game_textures.trail.clone();
            } else {
                player_sprite = game_textures.trail2.clone();
            }
            // Spawn trail
            commands
                .spawn_bundle(SpriteBundle {
                    texture: player_sprite,
                    transform: Transform {
                        translation: player_pos,
                        scale: Vec3::new(0.2, 0.2, 0.8),
                        rotation: player_rot,
                    },
                    ..Default::default()
                })
                .insert(Trail {
                    spawn_time: time.seconds_since_startup(),
                })
                .insert(Collider);
        }
    }
}

fn despawn_player_trail_system(
    mut commands: Commands,
    query: Query<(Entity, &mut Trail)>,
    time: Res<Time>,
) {
    for (entity, trail) in query.iter() {
        let time_alive = time.seconds_since_startup() - trail.spawn_time;
        if time_alive > 2. {
            commands.entity(entity).despawn();
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut velocity: Query<(&mut Velocity, &Player)>,
    mut rotvel: Query<(&mut RotationalVelocity, &Player)>,
) {
    for (mut rotvel, player) in rotvel.iter_mut() {
        manage_player_rotvel_keys(&kb, &mut rotvel.omega, player.id);
    }
    for (mut velocity, player) in velocity.iter_mut() {
        manage_player_velocity_keys(&kb, &mut velocity.velocity, player.id);
    }
}

fn manage_player_rotvel_keys(kb: &Res<Input<KeyCode>>, omega: &mut f32, id: u8) {
    if id == 1 {
        *omega += if kb.pressed(KeyCode::D) {
            -1.5
        } else if kb.pressed(KeyCode::A) {
            1.5
        } else {
            *omega *= 0.75;
            0.
        };
        *omega = omega.clamp(-3., 3.);
    } else if id == 2 {
        *omega += if kb.pressed(KeyCode::L) {
            -1.5
        } else if kb.pressed(KeyCode::J) {
            1.5
        } else {
            *omega *= 0.75;
            0.
        };
        *omega = omega.clamp(-3., 3.);
    }
}

fn manage_player_velocity_keys(kb: &Res<Input<KeyCode>>, velocity: &mut f32, id: u8) {
    if id == 1 {
        *velocity += if kb.pressed(KeyCode::S) {
            // Decelerate
            *velocity *= 0.75;
            0.
        } else if kb.pressed(KeyCode::W) {
            // accelerate
            1.
        } else {
            // if no key pressed then dont change the speed
            0.
        };
        // Apply max velocity
        *velocity = velocity.clamp(1.5, 3.);
    } else if id == 2 {
        *velocity += if kb.pressed(KeyCode::K) {
            // Decelerate
            *velocity *= 0.75;
            0.
        } else if kb.pressed(KeyCode::I) {
            // accelerate
            1.
        } else {
            // if no key pressed then dont change the speed
            0.
        };
        // Apply max velocity
        *velocity = velocity.clamp(1.5, 3.);
    }
}

fn player_movement_system(
    mut query: Query<(&Velocity, &RotationalVelocity, &mut Transform), With<Player>>,
) {
    for (velocity, rotvel, mut transform) in query.iter_mut() {
        let delta_rotation = Quat::from_rotation_z(rotvel.omega * TIME_STEP * ROTATION_SPEED);
        transform.rotation *= delta_rotation;
        let movement_vector = transform.rotation * Vec3::Y;
        let movement_distance = velocity.velocity * TIME_STEP * BASE_SPEED;
        let translation_delta = movement_vector * movement_distance;
        transform.translation += translation_delta;
        transform.translation.x = transform.translation.x.clamp(-500., 500.);
        transform.translation.y = transform.translation.y.clamp(-500., 500.);
    }
}

fn player_collition_system(
    collider_query: Query<&Transform, With<Collider>>,
    player_query: Query<&Transform, With<Player>>,
    mut exit: EventWriter<AppExit>,
) {
    for player_transform in player_query.iter() {
        for collider_transform in collider_query.iter() {
            let collision = collide(
                player_transform.translation,
                Vec2::splat(1.5),
                collider_transform.translation,
                Vec2::splat(1.5),
            );
            if let Some(collision) = collision {
                exit.send(AppExit);
            }
        }
    }
}

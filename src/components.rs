use bevy::prelude::Component;

// common components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// Player component
#[derive(Component)]
pub struct Player;

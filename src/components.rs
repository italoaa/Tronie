use bevy::prelude::Component;

// common components
#[derive(Component)]
pub struct Velocity {
    pub velocity: f32,
}

#[derive(Component)]
pub struct RotationalVelocity {
    pub omega: f32,
}

// Player component
#[derive(Component)]
pub struct Player {
    pub id: u8,
}

#[derive(Component)]
pub struct Trail {
    pub spawn_time: f64,
}

#[derive(Component)]
pub struct Collider;

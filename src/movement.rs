use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug)]
pub struct Acceleration(pub Vec3);

#[derive(Bundle)]
pub struct MoveableObjectBundle {
    pub vel: Velocity,
    pub acc: Acceleration,
    pub model: SceneBundle,
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (vel, mut transform) in query.iter_mut() {
        transform.translation += vel.0 * dt;
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (acc, mut vel) in query.iter_mut() {
        vel.0 += acc.0 * dt;
    }
}


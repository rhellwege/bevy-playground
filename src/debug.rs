use bevy::prelude::*;

use crate::movement::{Acceleration, MoveableObjectBundle, Velocity};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_movables);
    }
}

fn log_movables(query: Query<(&Transform, &Velocity, &Acceleration)>) {
    for (t, v, a) in query.iter() {
        info!("Entity at {:?}\tVelocity {:?}\tAcceleration {:?}", t, v, a);
    }
}


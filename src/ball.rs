use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MoveableObjectBundle, Velocity},
};

const INITIAL_VELOCITY: Vec3 = Vec3::new(-0.5, 0., 0.);
const INITIAL_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);
const SCALE: Vec3 = Vec3::new(1., 1., 1.);

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ball);
    }
}

fn spawn_ball(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    info!("spawning ball...");
    commands.spawn((
        Ball,
        MoveableObjectBundle {
            acc: Acceleration(Vec3::ZERO),
            vel: Velocity(INITIAL_VELOCITY),
            model: SceneBundle {
                // cloning the handle increments the Reference Count
                scene: scene_assets.ball.clone(),
                transform: Transform {
                    translation: INITIAL_TRANSLATION,
                    scale: SCALE,
                    ..default()
                },
                ..default()
            },
        },
    ));
}


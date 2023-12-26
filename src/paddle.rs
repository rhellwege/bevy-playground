use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MoveableObjectBundle, Velocity},
};

const PLAYER_TRANSLATION: Vec3 = Vec3::new(-50., 0., 0.);
const ENEMY_TRANSLATION: Vec3 = Vec3::new(50., 0., 0.);
const PADDLE_SCALE: Vec3 = Vec3::new(5., 5., 5.);

#[derive(Component)]
pub struct PlayerPaddle;

#[derive(Component)]
pub struct EnemyPaddle;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (spawn_player_paddle, spawn_enemy_paddle));
    }
}

fn spawn_player_paddle(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    info!("spawning player paddle...");
    commands.spawn((
        PlayerPaddle,
        MoveableObjectBundle {
            acc: Acceleration(Vec3::ZERO),
            vel: Velocity(Vec3::ZERO),
            model: SceneBundle {
                // cloning the handle increments the Reference Count
                scene: scene_assets.paddle.clone(),
                transform: Transform {
                    translation: PLAYER_TRANSLATION,
                    scale: PADDLE_SCALE,
                    ..default()
                },
                ..default()
            },
        },
    ));
}

fn spawn_enemy_paddle(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    info!("spawning enemy paddle...");
    commands.spawn((
        EnemyPaddle,
        MoveableObjectBundle {
            acc: Acceleration(Vec3::ZERO),
            vel: Velocity(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.paddle.clone(),
                transform: Transform {
                    translation: ENEMY_TRANSLATION,
                    scale: PADDLE_SCALE,
                    ..default()
                },
                ..default()
            },
        },
    ));
}


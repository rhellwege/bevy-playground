mod asset_loader;
mod ball;
mod camera;
mod debug;
mod movement;
mod paddle;
use asset_loader::AssetLoaderPlugin;
use ball::BallPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use paddle::PaddlePlugin;

fn main() {
    App::new()
        // resources / builtins
        .insert_resource(ClearColor(Color::rgb(0.31, 0.0, 0.45)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // app plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BallPlugin)
        // .add_plugins(DebugPlugin)
        // update
        .add_plugins(MovementPlugin)
        .run();
}

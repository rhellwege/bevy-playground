/// Loads globally shared asset handles
use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub paddle: Handle<Scene>,
    pub ball: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    info!("Loading assets...");
    *scene_assets = SceneAssets {
        paddle: asset_server.load("Snowboard.glb#Scene0"),
        ball: asset_server.load("Planet.glb#Scene0"),
    };
    info!("Successfully loaded assets.");
}


use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub lungi: Handle<Image>,
    pub lungi_jumps: Handle<Image>,
    pub money_obstacle: Handle<Image>,
    pub hotdog_obstacle: Handle<Image>,
    pub bg: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        lungi: asset_server.load("Lungi.png"),
        lungi_jumps: asset_server.load("Lungi_jumps.png"),
        money_obstacle: asset_server.load("money_obstacle.png"),
        hotdog_obstacle: asset_server.load("hotdog_obstacle.png"),
        bg: asset_server.load("bg.png"),
    }
}

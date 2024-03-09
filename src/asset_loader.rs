use bevy::prelude::*;

const SPACESHIP_MODEL_PATH: &'static str = "Spaceship.glb#Scene0";
const ASTEROID_MODEL_PATH: &'static str = "Planet.glb#Scene0";
const BULLET_MODEL_PATH: &'static str = "Bullet.glb#Scene0";



#[derive(Resource, Default, Debug)]
pub struct SceneAssets {
    pub spaceship: Handle<Scene>,
    pub asteroid: Handle<Scene>,
    pub bullet: Handle<Scene>,
}


pub struct AssetLoaderPlugin;


impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}


fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        spaceship: asset_server.load(SPACESHIP_MODEL_PATH),
        asteroid: asset_server.load(ASTEROID_MODEL_PATH),
        bullet: asset_server.load(BULLET_MODEL_PATH),
    }
}

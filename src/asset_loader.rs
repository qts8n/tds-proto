use bevy::prelude::*;
use rand::seq::IteratorRandom;

const SPACESHIP_MODEL_PATHS: &'static [&'static str] = &[
    "Spaceship-1.glb#Scene0",
    "Spaceship-2.glb#Scene0",
    "Spaceship-3.glb#Scene0",
    "Spaceship-4.glb#Scene0",
];
const ASTEROID_MODEL_PATH: &'static [&'static str] = &[
    "Planet-1.glb#Scene0",
    "Planet-2.glb#Scene0",
    "Planet-3.glb#Scene0",
    "Planet-4.glb#Scene0",
];
const BULLET_MODEL_PATH: &'static [&'static str] = &[
    "Bullet-1.glb#Scene0",
];


#[derive(Resource, Default, Debug)]
pub struct SceneAssets {
    pub spaceship: Vec<Handle<Scene>>,
    pub asteroid: Vec<Handle<Scene>>,
    pub bullet: Vec<Handle<Scene>>,
}


impl SceneAssets {
    pub fn get_random_spaceship(&self) -> Handle<Scene> {
        self.spaceship.iter().choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn get_random_asteroid(&self) -> Handle<Scene> {
        self.asteroid.iter().choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn get_random_bullet(&self) -> Handle<Scene> {
        self.bullet.iter().choose(&mut rand::thread_rng()).unwrap().clone()
    }
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
        spaceship: SPACESHIP_MODEL_PATHS.iter().map(|path| asset_server.load(path.to_string())).collect(),
        asteroid: ASTEROID_MODEL_PATH.iter().map(|path| asset_server.load(path.to_string())).collect(),
        bullet: BULLET_MODEL_PATH.iter().map(|path| asset_server.load(path.to_string())).collect(),
    }
}

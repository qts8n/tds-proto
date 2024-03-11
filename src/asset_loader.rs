use bevy::prelude::*;
use rand::seq::IteratorRandom;

const SPACESHIP_MODEL_PATHS: &'static [&'static str] = &[
    "Spaceship-1.glb#Scene0",
    "Spaceship-2.glb#Scene0",
    "Spaceship-3.glb#Scene0",
    "Spaceship-4.glb#Scene0",
];
const ASTEROID_MODEL_PATH: &'static [&'static str] = &[
    "Planet-fractured-1.glb#Scene0",
    "Planet-fractured-2.glb#Scene0"
];
const BULLET_MODEL_PATH: &'static [&'static str] = &[
    "Bullet-1.glb#Scene0",
];

const PLAY_ICON_PATH: &'static str = "Right.png";
const EXIT_ICON_PATH: &'static str = "Exit.png";


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


#[derive(Resource, Default, Debug)]
pub struct ImageAssets {
    pub play_icon: Handle<Image>,
    pub exit_icon: Handle<Image>,
}


pub struct AssetLoaderPlugin;


impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SceneAssets>()
            .init_resource::<ImageAssets>()
            .add_systems(Startup, (
                load_scene_assets,
                load_image_assets,
            ));
    }
}


fn load_scene_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        spaceship: SPACESHIP_MODEL_PATHS.iter().map(|path| asset_server.load(path.to_string())).collect(),
        asteroid: ASTEROID_MODEL_PATH.iter().map(|path| asset_server.load(path.to_string())).collect(),
        bullet: BULLET_MODEL_PATH.iter().map(|path| asset_server.load(path.to_string())).collect(),
    };
}


fn load_image_assets(mut image_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *image_assets = ImageAssets {
        play_icon: asset_server.load(PLAY_ICON_PATH),
        exit_icon: asset_server.load(EXIT_ICON_PATH),
    };
}

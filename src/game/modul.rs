
#[allow(unused)]
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use snake::SnakePlugin;
use food::FoodPlugin;
use crate::components::{GameOverEvent};

#[path = "snake.rs"]
mod snake;
//#[path = "food.rs"]
mod food;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_plugin(SnakePlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(AudioPlugin)
            .add_startup_system(start_background_audio);
            

    }}


    pub fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
        audio
            .play(asset_server.load("sounds/hell.ogg".to_string()))
            .looped();
    }
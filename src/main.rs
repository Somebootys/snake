#[allow(unused)]
use bevy::prelude::*;
//use bevy::time::FixedTimestep;
//use rand::prelude::random;

use crate::components::{GrowthEvent, Position, Size};
use crate::food::FoodPlugin;
use crate::snake::SnakePlugin;
mod components;
mod food;
mod snake;

//Constants ---------
const SNAKE_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

const TIMESTEP_3_PER_SECOND: f64 = 60.0 / 180.0;
//const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;
//Constants end ------

fn camera_setup_system(mut command: Commands) {
    command.spawn_bundle(Camera2dBundle::default());
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            // <--
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .add_startup_system(camera_setup_system)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugin(SnakePlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(FoodPlugin)
        .run();
}

/*

Useful references

[1]
Titel = 'How do I check if a thing is in a vector'
url =https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector
*/

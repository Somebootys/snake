#[allow(unused)]
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
//use bevy::time::FixedTimestep;
//use rand::prelude::random;

use crate::components::{GameOverEvent, Position, Score, ScoreText, Size};
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

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/hell.ogg".to_string()))
        .looped();
}

fn camera_setup_system(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn(Camera2dBundle::default());

    command.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                },
            ),
        ])
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            ..default()
        }),
        ScoreText,
    ));
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
        .add_event::<GameOverEvent>()
        .add_startup_system(camera_setup_system)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugin(SnakePlugin)
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Snake Game".to_string(),
                        width: 500.0,
                        height: 500.0,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(FoodPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(start_background_audio)
        .insert_resource(Score(0))
        .run();
}

/*

Useful references

[1]
Titel = 'How do I check if a thing is in a vector'
url =https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector
*/

#[allow(unused)]
use bevy::prelude::*;
//use bevy::time::FixedTimestep;
use rand::prelude::random;

use crate::components::{Food, GrowthEvent, LastTailPosition, Position, Size, SnakeHead};
use crate::snake::{spawn_snake_segment, SnakeSegments};
use crate::{ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_food)
            .add_event::<GrowthEvent>()
            .add_system(spawn_new_food)
            .add_system(snake_growth.after(spawn_new_food));
    }
}

fn spawn_food(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn spawn_new_food(
    query_food: Query<(Entity, &Position), With<Food>>,
    head_query: Query<&Position, With<SnakeHead>>,
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
) {
    if let Some((food_entity, food_pos)) = query_food.iter().next() {
        if let Some(head_pos) = head_query.iter().next() {
            if collision_check(head_pos.x, head_pos.y, food_pos.x, food_pos.y) {
                println!("Yum!");
                commands.entity(food_entity).despawn();
                growth_writer.send(GrowthEvent);
                spawn_food(commands);
            }
        }
    }
}

fn collision_check(head_x: i32, head_y: i32, food_x: i32, food_y: i32) -> bool {
    head_x == food_x && food_y == head_y
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_snake_segment(commands, last_tail_position.0.unwrap()));
    }
}
/*
Hi there

*/

#[allow(unused)]
use bevy::prelude::*;
use bevy::time::FixedTimestep;

use crate::components::{Direction, LastTailPosition, Position, Size, SnakeHead, SnakeSegment};
use crate::{SNAKE_COLOR, SNAKE_SEGMENT_COLOR, TIMESTEP_3_PER_SECOND};

#[derive(Default, Deref, DerefMut)]
pub struct SnakeSegments(Vec<Entity>);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_snake)
            .add_system(snake_movement_input.before(snake_movement)) // ensure func 2 is run before func 1??
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIMESTEP_3_PER_SECOND))
                    .with_system(snake_movement),
            )
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default());
    }
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::UP,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_snake_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut lasttailpos: ResMut<LastTailPosition>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|entity| *positions.get_mut(*entity).unwrap())
            .collect::<Vec<Position>>();

        *lasttailpos = LastTailPosition(Some(*segment_positions.last().unwrap()));

        let mut head_pos = positions.get_mut(head_entity).unwrap();

        // thanks SO [1] - maybe there is a better way? We are checking if any
        // of the snake segments [1..] are equal to the snake head, [0]
        {
            let mut snakesegments = segment_positions[1..].iter();
            let snakehead = segment_positions[0];

            if snakesegments.any(|&pos| pos == snakehead) {
                println!("Game over");
            }
        }
        //println!("head posiotions: {:?} \n segment pos: {:?}", head_pos, segment_positions);

        match &head.direction {
            Direction::LEFT => {
                head_pos.x -= 1;
            }
            Direction::RIGHT => {
                head_pos.x += 1;
            }
            Direction::UP => {
                head_pos.y += 1;
            }
            Direction::DOWN => {
                head_pos.y -= 1;
            }
        };
        segment_positions
            .iter()
            .zip(segments.iter().skip(1)) //brings two collection together. Swap to the left continously, like array.
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::LEFT
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::DOWN
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::UP
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::RIGHT
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn spawn_snake_segment(mut commands: Commands, pos: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(pos)
        .insert(Size::square(0.5))
        .id() // returns an id whenever called, notice no semi colon, hence return

    // instructions only ends at the last expression, one().two().andso_on() <--last one
}

#[allow(unused)]
use bevy::prelude::*;

//Components--------------
#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct Size {
    pub height: f32,
    pub width: f32,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Food;

#[derive(Component, Debug)]
pub struct SnakeSegment;

#[derive(Component, Debug)]
pub struct GrowthEvent;

#[derive(Component, Debug)]
pub struct GameOverEvent;

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

//Components end-----------

//Implementations for Components

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            height: x,
            width: x,
        }
    }
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::RIGHT => Self::LEFT,
            Self::LEFT => Self::RIGHT,
            Self::UP => Self::DOWN,
            Self::DOWN => Self::UP,
        }
    }
}

// implementation end ----------

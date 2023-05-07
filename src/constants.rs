use bevy::prelude::*;
//C onstants ---------
pub const SNAKE_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const FOOD_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);
 
pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;
 
pub const TIMESTEP_3_PER_SECOND: f64 = 60.0 / 180.0;
 
//Constants end ------
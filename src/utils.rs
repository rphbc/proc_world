use bevy::{ecs::{system::Query, query::With}, window::{Window, PrimaryWindow}};

use crate::configs::*;

pub fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_W as f32 * SPRITE_SCALE_FACTOR as f32,
        y * TILE_H as f32 * SPRITE_SCALE_FACTOR as f32,
    )
}

pub fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        let (x,y) = grid_to_world(position.x as f32, position.y as f32);
        println!("Cursor is inside the primary window, at {:?}-{:?}", x, y);
    } else {
        println!("Cursor is not in the game window.");
    }
}

pub fn limit_var ( mut v: f32, limit: f32) -> f32 {
    if v > limit {
        v = limit;
    }

    if v < -limit {
        v = -limit;
    }
    v
}
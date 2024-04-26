mod grid;
use std::ffi::CString;

use crate::grid::*;
use raylib_ffi::{
    colors, BeginDrawing, ClearBackground, DrawFPS, DrawRectangle, EndDrawing, WindowShouldClose,
};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BOX_DIMENSION: usize = 10;

fn main() {
    unsafe {
        let window_name = CString::new("Game of life").unwrap();
        raylib_ffi::InitWindow(WIDTH as i32, HEIGHT as i32, window_name.as_ptr());
    }

    let box_width: usize = WIDTH / BOX_DIMENSION;
    let box_height: usize = HEIGHT / BOX_DIMENSION;

    let mut gr = SetGrid::new(box_width, box_height, 0.5);

    while !unsafe { WindowShouldClose() } {
        unsafe {
            BeginDrawing();
            ClearBackground(colors::BLACK);
        }

        for i in 0..box_width {
            for j in 0..box_height {
                unsafe {
                    DrawRectangle(
                        (BOX_DIMENSION * i) as i32,
                        (BOX_DIMENSION * j) as i32,
                        (BOX_DIMENSION - 1) as i32,
                        (BOX_DIMENSION - 1) as i32,
                        if gr.get_elem(i as u64, j as u64) == CellState::Alive {
                            colors::WHITE
                        } else {
                            colors::BLACK
                        },
                    )
                }
            }
        }

        unsafe {
            DrawFPS(10, 10);
        }

        gr.run_gol_step();

        unsafe {
            EndDrawing();
        }
    }
}

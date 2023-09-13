use raylib::prelude::*;
mod grid;
use crate::grid::grid::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BOX_DIMENSION: usize = 10;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Game of life")
        .build();

    let box_width: usize = WIDTH / BOX_DIMENSION;
    let box_height: usize = HEIGHT / BOX_DIMENSION;

    rl.set_target_fps(30);

    let mut gr = SetGrid::new(box_width, box_height, 0.5);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for i in 0..box_width {
            for j in 0..box_height {
                d.draw_rectangle(
                    (BOX_DIMENSION * i) as i32,
                    (BOX_DIMENSION * j) as i32,
                    (BOX_DIMENSION - 1) as i32,
                    (BOX_DIMENSION - 1) as i32,
                    if gr.get_elem(i as u64, j as u64) == CellState::Alive {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    },
                );
            }
        }

        // d.draw_text("Hello world", 200, 200, 20, Color::BLACK);
        d.draw_fps(10, 10);

        gr.run_gol_step();
    }
}

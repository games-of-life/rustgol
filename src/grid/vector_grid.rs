use super::grid_trait::*;
use rand::Rng;

pub struct VectorGrid {
    dims: Size,
    field: Vec<CellState>,
}

impl VectorGrid {
    pub fn new(width: usize, height: usize, prob: f64) -> Self {
        Self {
            dims: Size {
                height: height,
                width: width,
            },
            field: (0..width * height)
                .map(|_| {
                    if rand::thread_rng().gen::<f64>() > prob {
                        CellState::Dead
                    } else {
                        CellState::Alive
                    }
                })
                .collect(),
        }
    }

    fn calc_neighbors(&self, i: u64, j: u64) -> u64 {
        let mut count: u64 = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                let n_x: i64 = i as i64 + dx;
                let n_y: i64 = j as i64 + dy;
                if !((dx == 0 && dy == 0)
                    || n_x < 0
                    || n_y < 0
                    || n_x >= self.dims.width as i64
                    || n_y >= self.dims.height as i64)
                    && (self.get_elem(n_x as u64, n_y as u64) == CellState::Alive)
                {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Grid for VectorGrid {
    fn get_elem(&self, i: u64, j: u64) -> CellState {
        self.field[i as usize * self.dims.height + j as usize]
    }

    fn set_elem(&mut self, i: u64, j: u64, val: CellState) {
        self.field[i as usize * self.dims.height + j as usize] = val;
    }

    fn run_gol_step(&mut self) {
        let mut new_field = self.field.clone();

        for i in 0..self.dims.width {
            for j in 0..self.dims.height {
                let neigh = self.calc_neighbors(i as u64, j as u64);
                if 3 == neigh
                    || (2 == neigh && CellState::Alive == self.get_elem(i as u64, j as u64))
                {
                    new_field[i * self.dims.height + j] = CellState::Alive
                } else {
                    new_field[i * self.dims.height + j] = CellState::Dead
                }
            }
        }

        self.field = new_field;
    }
}

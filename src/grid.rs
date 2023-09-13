
use itertools::Itertools;
use rand::Rng;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
}

pub trait Grid {
    fn get_elem(&self, i: u64, j: u64) -> CellState;
    fn set_elem(&mut self, i: u64, j: u64, val: CellState);
    fn run_gol_step(&mut self);
}

pub struct Size {
    width: usize,
    height: usize,
}

pub struct VectorGrid {
    dims: Size,
    field: Vec<CellState>,
}

pub struct SetGrid {
    dims: Size,
    field: HashSet<(u64, u64)>,
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

impl SetGrid {
    pub fn new(width: usize, height: usize, prob: f64) -> Self {
        Self {
            dims: Size {
                width: width,
                height: height,
            },
            field: (0..(width as f64 * height as f64 * prob) as usize)
                .map(|_| {
                    (
                        rand::thread_rng().gen_range(0..width as u64),
                        rand::thread_rng().gen_range(0..height as u64),
                    )
                })
                .collect(),
        }
    }
}

impl Grid for SetGrid {
    fn get_elem(&self, i: u64, j: u64) -> CellState {
        if self.field.contains(&(i, j)) {
            CellState::Alive
        } else {
            CellState::Dead
        }
    }

    fn set_elem(&mut self, i: u64, j: u64, val: CellState) {
        match val {
            CellState::Alive => self.field.insert((i, j)),
            CellState::Dead => self.field.remove(&(i, j)),
        };
    }

    fn run_gol_step(&mut self) {
        let moore = |k: (u64, u64)| {
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|x| !(x.0 == 0 && x.1 == 0))
                .map(move |x| (x.0 + k.0 as i64, x.1 + k.1 as i64))
        };

        let valuable_points = self.field.iter().flat_map(|x| moore(*x)).filter(|x| {
            !(x.0 < 0 || x.1 < 0 || x.0 >= self.dims.width as i64 || x.1 >= self.dims.height as i64)
        });

        let frequencies_binding = valuable_points.sorted().group_by(|x| x.clone());

        let frequencies = frequencies_binding
            .into_iter()
            .map(|(elem, gr)| (elem, gr.count()));

        let good_points = frequencies.filter_map(|x| {
            if x.1 == 3
                || (x.1 == 2 && self.get_elem(x.0 .0 as u64, x.0 .1 as u64) == CellState::Alive)
            {
                Some((x.0 .0 as u64, x.0 .1 as u64))
            } else {
                None
            }
        });

        let new_field: HashSet<(u64, u64)> = good_points.collect();

        self.field = new_field;
    }
}

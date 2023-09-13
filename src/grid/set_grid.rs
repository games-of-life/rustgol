use super::grid_trait::*;
use std::collections::HashSet;
use rand::Rng;
use itertools::Itertools;

pub struct SetGrid {
    dims: Size,
    field: HashSet<(u64, u64)>,
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

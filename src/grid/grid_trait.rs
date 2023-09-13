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
    pub width: usize,
    pub height: usize,
}

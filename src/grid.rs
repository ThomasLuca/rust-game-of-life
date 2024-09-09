use rand::prelude::*;

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(w: usize, h: usize) -> Grid {
        Grid {
            width: w,
            height: h,
            cells: vec![vec![false; w]; h],
        }
    }

    pub fn randomize(&mut self, coverage: f64) {
        let mut rng = rand::thread_rng();
        for row in &mut self.cells {
            for cell in row {
                *cell = rng.gen_bool(coverage);
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.cells[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        self.cells[y][x] = state;
    }
}

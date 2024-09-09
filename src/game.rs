use crate::grid::Grid;

pub struct Game {
    pub grid: Grid,
}

impl Game {
    pub fn new(grid: Grid) -> Self {
        Game { grid }
    }

    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let n_neighbors = self.count_neighbors(x, y);
                let is_alive = self.grid.get(x, y);

                let next_state = match (is_alive, n_neighbors) {
                    (true, 2) | (_, 3) => true, // Survive or reproduce
                    _ => false,                 // Die
                };

                new_grid.set(x, y, next_state);
            }
        }
        self.grid = new_grid;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for i in (x.saturating_sub(1))..=(x + 1).min(self.grid.width - 1) {
            for j in (y.saturating_sub(1))..=(y + 1).min(self.grid.height - 1) {
                if (i != x || j != y) && self.grid.get(i, j) {
                    count += 1;
                }
            }
        }
        count
    }
}

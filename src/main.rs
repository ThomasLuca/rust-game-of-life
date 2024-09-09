mod display;
mod game;
mod grid;
use crate::{game::Game, grid::Grid};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use display::display;
use std::io::stdout;
use std::time::Duration;

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut grid = Grid::new(80, 30);
    grid.randomize(0.20);

    let mut game = Game::new(grid);

    loop {
        execute!(stdout(), Clear(ClearType::All)).expect("Failed to clear terminal");
        display(&game.grid);
        game.update();
        if event::poll(Duration::from_millis(10)).expect("Polling failed") {
            if let Event::Key(key_event) = event::read().expect("Failed to read event") {
                // Check if the user pressed the 'q' key
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500))
    }
    disable_raw_mode().expect("Failed to disable raw mode");
}

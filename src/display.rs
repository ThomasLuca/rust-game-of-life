use crate::grid::Grid;

pub fn display(grid: &Grid) {
    println!("╔{}╗", "══".repeat(grid.width));
    print!("\r");
    for y in 0..grid.height {
        print!("║");
        for x in 0..grid.width {
            print!("{}", if grid.get(x, y) { "██" } else { "  " });
        }
        println!("║");
        print!("\r");
    }
    println!("╚{}╝", "══".repeat(grid.width));
    print!("\r");
}

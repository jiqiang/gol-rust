use std::collections::HashMap;

const MAX_NUM_OF_X: u32 = 5;
const MAX_NUM_OF_Y: u32 = 5;

fn reset_grid() -> HashMap<(u32, u32), bool> {
    let mut map = HashMap::new();

    for x in 0..MAX_NUM_OF_X {
        for y in 0..MAX_NUM_OF_Y {
            map.insert((x, y), false);
        }
    }

    map
}

fn set_initial_pattern(grid: &mut HashMap<(u32, u32), bool>, cells: Vec<(u32, u32)>) {
    for c in cells.iter() {
        grid.insert((c.0, c.1), true);
    }
}

fn main() {
    
    let mut grid = reset_grid();

    let cells = vec![(2,2)];

    set_initial_pattern(&mut grid, cells);

    println!("{:?}", grid);
}

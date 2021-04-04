const HALF_SIZE: usize = 30;
const SIZE: usize = HALF_SIZE * 2 + 1;

const MOVES: [(isize, isize); 8] = [
    (1, 2),
    (2, 1),
    (1, -2),
    (2, -1),
    (-1, 2),
    (-2, 1),
    (-1, -2),
    (-2, -1),
];

macro_rules! idx {
    ($x:expr, $y:expr) => {
        $x + $y * SIZE
    };
}

macro_rules! set {
    ($grid:ident, $x:ident, $y:ident, $i:ident) => {
        $i += 1;
        $grid[idx!($x, $y)] = $i;
    };
}

fn main() {
    let mut grid = [0usize; SIZE * SIZE];

    populate_grid(&mut grid);
    let path = escape(&grid);

    let max_idx = dbg!(path.len()) - 1;
    dbg!(grid[idx!(path[max_idx].0, path[max_idx].1)]);
}

fn populate_grid(grid: &mut [usize; SIZE * SIZE]) {
    let mut i = 0;
    let mut side_len = 0;

    let mut x = HALF_SIZE;
    let mut y = HALF_SIZE;

    while x < SIZE - 1 {
        x += 1;
        y -= 1;

        side_len += 2;

        for _ in 0..side_len {
            y += 1;
            set!(grid, x, y, i);
        }

        for _ in 0..side_len {
            x -= 1;
            set!(grid, x, y, i);
        }

        for _ in 0..side_len {
            y -= 1;
            set!(grid, x, y, i);
        }

        for _ in 0..side_len {
            x += 1;
            set!(grid, x, y, i);
        }
    }
}

fn escape(grid: &[usize; SIZE * SIZE]) -> Vec<(usize, usize)> {
    let mut visited = [false; SIZE * SIZE];
    let mut path = Vec::new();

    let mut x = HALF_SIZE;
    let mut y = HALF_SIZE;

    loop {
        visited[idx!(x, y)] = true;
        path.push((x, y));

        let mut next: Option<(usize, usize, usize)> = None;

        for (x_mov, y_mov) in &MOVES {
            let pos_x = (x as isize + x_mov) as usize;
            let pos_y = (y as isize + y_mov) as usize;
            let idx = idx!(pos_x, pos_y);

            if !visited[idx] {
                let pos_square = grid[idx];

                if !matches!(next, Some((square, _, _)) if square < pos_square) {
                    next = Some((pos_square, pos_x, pos_y));
                }
            }
        }

        if let Some((_, next_x, next_y)) = next {
            x = next_x;
            y = next_y;
        } else {
            println!("Stuck at ({}, {})", x, y);
            return path;
        }
    }
}

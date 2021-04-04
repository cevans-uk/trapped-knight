use std::io;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

const HALF_SIZE: usize = 30;
const SIZE: usize = HALF_SIZE * 2 + 1;

const SQUARE_HALF_SIZE: usize = 3;
const SQUARE_SIZE: usize = SQUARE_HALF_SIZE * 2 + 1;

const SVG_HALF_SIZE: usize = (SQUARE_SIZE * SIZE) / 2;
const SVG_SIZE: usize = SVG_HALF_SIZE * 2 + 1;

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

    write_svg(&path);
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
            eprintln!("Trapped at ({}, {})", x, y);
            return path;
        }
    }
}

fn write_svg(path: &[(usize, usize)]) {
    let mut document = Document::new().set("viewBox", (0, 0, SVG_SIZE, SVG_SIZE));

    let mut hue: f32 = 0.0;
    let mut iter = path.windows(2);

    while let Some(&[(x1, y1), (x2, y2)]) = iter.next() {
        let data = Data::new()
            .move_to((x1 * SQUARE_SIZE, y1 * SQUARE_SIZE))
            .line_to((x2 * SQUARE_SIZE, y2 * SQUARE_SIZE));

        let stroke = format!("hsl({}, 100%, 70%", hue);

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", stroke)
            .set("stroke-width", 2)
            .set("d", data);

        document = document.add(path);

        hue += 0.11;
    }

    svg::write(io::stdout().lock(), &document).unwrap();
}

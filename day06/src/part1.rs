#![allow(unused)]
use array2d::Array2D;
use itertools::Itertools;
use miette::IntoDiagnostic;
use tracing::info;

#[derive(Default, Debug)]
enum Facing {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Debug)]
struct Guard {
    x: i32,
    y: i32,
    facing: Facing,
}

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day06".to_uppercase());
    // let lines: Vec<String> = test.lines().map(|s| s.to_owned()).collect();
    let lines: Vec<&str> = input.lines().collect_vec();
    // let lines: Vec<&str> = test.lines().collect_vec();

    let mut guard = Guard::default();

    let lines: Vec<Vec<bool>> = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        guard.x = x as i32;
                        guard.y = y as i32;
                    };
                    if c == '#' {
                        true
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect();
    // obstacles in the map are 'true'
    let map: Array2D<bool> = Array2D::from_rows(&lines).into_diagnostic()?;

    // unvisited are 0 visted are 1
    let mut visited: Array2D<i32> = Array2D::filled_with(0, map.num_rows(), map.num_columns());

    // guard.set(0, 4);
    // info!("{:?}", guard);
    loop {
        visited.set(guard.y as usize, guard.x as usize, 1);
        let next = guard.next_cell();
        // info!("Next: {:?}", next);

        let out_of_bounds = Guard::out_of_bounds(&map, next);

        if out_of_bounds {
            // println!("{:#?}", "Guard out of bounds");
            break;
        }

        if guard.path_obstructed(&map) {
            // info!("{:?}", "PATH OBSTRUCTED");
            guard.turn();
            // info!("{:?}", guard);
        } else {
            guard.set(next.0, next.1);
            // visited.set(guard.y as usize, guard.x as usize, 1);
            // info!("{:?}", guard);
        }
    }

    let count_visited_positions: i32 = visited.elements_row_major_iter().sum();

    Ok(count_visited_positions.to_string())
}

impl Guard {
    fn set(&mut self, x: impl Into<i32>, y: impl Into<i32>) {
        self.x = x.into();
        self.y = y.into();
    }
    fn turn(&mut self) {
        self.facing = match self.facing {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
    fn next_cell(&self) -> (i32, i32) {
        match self.facing {
            Facing::Up => (self.x, self.y - 1),
            Facing::Right => (self.x + 1, self.y),
            Facing::Down => (self.x, self.y + 1),
            Facing::Left => (self.x - 1, self.y),
        }
    }
    fn out_of_bounds(map: &Array2D<bool>, coords: (i32, i32)) -> bool {
        let (x, y) = coords;
        if x < 0 {
            return true;
        } else if x >= map.num_columns() as i32 {
            return true;
        } else if y < 0 {
            return true;
        } else if y >= map.num_rows() as i32 {
            return true;
        }
        false
    }
    fn path_obstructed(&self, map: &Array2D<bool>) -> bool {
        let next = self.next_cell();
        let obstructed = map.get(next.1 as usize, next.0 as usize).unwrap();
        obstructed.to_owned()
    }
}

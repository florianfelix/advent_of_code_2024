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
    x: usize,
    y: usize,
    facing: Facing,
}

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day06".to_uppercase());
    // let lines: Vec<String> = test.lines().map(|s| s.to_owned()).collect();
    let lines: Vec<&str> = test.lines().collect_vec();

    let mut guard = Guard::default();

    let lines: Vec<Vec<bool>> = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        guard.x = x;
                        guard.y = y;
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
    let visited: Array2D<i32> = Array2D::filled_with(0, map.num_rows(), map.num_columns());

    info!("{:?}", guard);

    let count_visited_positions: i32 = visited.elements_row_major_iter().sum();

    Ok(count_visited_positions.to_string())
}

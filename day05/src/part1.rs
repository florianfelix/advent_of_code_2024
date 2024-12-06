#![allow(unused)]
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day05".to_uppercase());
    let lines: Vec<String> = test.lines().map(|s| s.to_owned()).collect();

    let splitlines = lines.split(|l| l.is_empty()).collect_vec();

    let mut rules: Vec<(i32, i32)> = splitlines
        .first()
        .unwrap()
        .iter()
        .map(|l| {
            let r = l.split_once("|").unwrap();
            (r.0.parse::<i32>().unwrap(), r.1.parse::<i32>().unwrap())
        })
        .collect();

    let mut updates: Vec<Vec<i32>> = splitlines
        .last()
        .unwrap()
        .iter()
        .map(|l| {
            let r = l
                .split(",")
                .map(|e| e.parse::<i32>().unwrap())
                .collect_vec();
            r
        })
        .collect();

    info!("{:?}", rules);

    let result: i32 = updates
        .par_iter()
        .map(|u| middle_number(&rules, &u))
        .sum::<i32>();

    Ok(result.to_string())
}

fn middle_number(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> i32 {
    // if update correct return middle number else 0
    0
}

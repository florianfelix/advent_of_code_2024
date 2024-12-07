#![allow(unused)]
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day07".to_uppercase());
    let lines: Vec<(&str, &str)> = test
        .lines()
        .map(|s| s.split_once(":").unwrap())
        .collect_vec();
    let totest: Vec<i32> = lines
        .iter()
        .map(|l| l.0.parse::<i32>().unwrap())
        .collect_vec();
    let numbers = lines
        .iter()
        .map(|l| {
            l.1.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    info!("{:#?}", totest.first());

    let result: i32 = totest.iter().zip(numbers).map(|(t, n)| dotest(t, &n)).sum();

    Ok(result.to_string())
}

fn dotest(t: &i32, n: &Vec<i32>) -> i32 {
    info!("Numbers: {:?}", n);
    t.to_owned()
}

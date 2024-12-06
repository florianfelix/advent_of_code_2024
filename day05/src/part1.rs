#![allow(unused)]
use itertools::Itertools;
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
    let mut result: i32 = 0;

    Ok(result.to_string())
}

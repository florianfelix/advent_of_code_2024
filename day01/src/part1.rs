#![allow(unused)]
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day01".to_uppercase());
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    let mut left = vec![];
    let mut right = vec![];

    let _: Vec<_> = lines
        .iter()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|a| {
            left.push(a.first().unwrap().parse::<i32>().unwrap());
            right.push(a.last().unwrap().parse::<i32>().unwrap());
        })
        .collect();

    left.sort();
    right.sort();

    let mut sum: i32 = 0;

    for (l, r) in left.iter().zip(right) {
        let distance = (r - l).abs();
        sum += distance;
    }

    Ok(sum.to_string())
}

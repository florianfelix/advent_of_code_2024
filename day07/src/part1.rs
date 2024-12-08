#![allow(unused)]
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day07".to_uppercase());
    let lines: Vec<(&str, &str)> = input
        .lines()
        .map(|s| s.split_once(":").unwrap())
        .collect_vec();
    let totest: Vec<u64> = lines
        .iter()
        .map(|l| l.0.parse::<u64>().unwrap())
        .collect_vec();
    let numbers = lines
        .iter()
        .map(|l| {
            l.1.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let result: u64 = totest.iter().zip(numbers).map(|(t, n)| dotest(t, &n)).sum();

    Ok(result.to_string())
}

fn dotest(t: &u64, n: &Vec<u64>) -> u64 {
    let ops = n.len() - 1;
    let operands = vec!["+", "*"];
    let cart = (0..ops)
        .map(|_| operands.clone())
        .multi_cartesian_product()
        // .map(|p| p.1)
        .collect_vec();

    let result: Option<u64> = cart
        .iter()
        .any(|seq| {
            // println!("{:#?}", s);
            let mut s = seq.iter();
            let r = n
                .iter()
                .copied()
                .reduce(|acc, next_num| match s.next().unwrap() {
                    &"+" => acc + next_num,
                    &"*" => acc * next_num,
                    _ => panic!("Should not happen"),
                })
                .unwrap();
            r == *t
        })
        .then_some(t.clone());

    if result.is_none() {
        return 0_u64;
    } else {
        return result.unwrap();
    }
}

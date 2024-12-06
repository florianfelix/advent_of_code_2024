#![allow(unused)]
use day04::part1::process;
use miette::Context;
use tracing::{error, info};

#[tracing::instrument]
fn main() -> miette::Result<()> {
    day04::utils::trace();

    let file = include_str!("../../input/part1");
    let testfile = include_str!("../../input/part1test");

    let result = process(file, testfile);

    info!("{:?}", result);

    Ok(())
}
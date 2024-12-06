#![allow(unused)]
use day01::part1::process;
use miette::Context;
use tracing::{error, info};

#[tracing::instrument]
fn main() -> miette::Result<()> {
    day01::utils::trace();

    let file = include_str!("../../input/part1");

    let result = process(file);

    info!("{:?}", result);

    Ok(())
}

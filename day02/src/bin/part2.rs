#![allow(unused)]
use day02::part2::process;
use miette::Context;
use tracing::{error, info};

#[tracing::instrument]
fn main() -> miette::Result<()> {
    day02::utils::trace();

    let file = include_str!("../../input/part2test");

    let result = process(file);

    info!("{:?}", result);

    Ok(())
}

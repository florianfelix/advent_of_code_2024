#![allow(unused)]
use day08::part2::process;
use miette::Context;
use tracing::{error, info};

#[tracing::instrument]
fn main() -> miette::Result<()> {
    day08::utils::trace();

    let file = include_str!("../../input/part2");
    let testfile = include_str!("../../input/part2test");

    let result = process(file, testfile);

    info!("{:?}", result);

    Ok(())
}

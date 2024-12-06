#![allow(unused)]
use {{crate_name}}::part2::process;
use miette::Context;
use tracing::{error, info};

#[tracing::instrument]
fn main() -> miette::Result<()> {
    {{crate_name}}::utils::trace();

    let file = include_str!("../../input/part2test");

    let result = process(file);

    info!("{:?}", result);

    Ok(())
}

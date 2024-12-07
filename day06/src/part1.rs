#![allow(unused)]
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day06".to_uppercase());
    let lines: Vec<String> = test.lines().map(|s| s.to_owned()).collect();

    info!("{:#?}", lines);

    let mut positions: i32 = 0;

    Ok(positions.to_string())
}

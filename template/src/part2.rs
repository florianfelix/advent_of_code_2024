#![allow(unused)]
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    info!("{} - PART 2", "{{crate_name}}".to_uppercase());
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    info!("{:#?}", lines);

    Ok("RESULT".into())
}

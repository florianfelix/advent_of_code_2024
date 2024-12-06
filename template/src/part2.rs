#![allow(unused)]
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 2", "{{crate_name}}".to_uppercase());
    let lines: Vec<String> = test.lines().map(|s| s.to_owned()).collect();

    info!("{:#?}", lines);

    Ok("RESULT".into())
}

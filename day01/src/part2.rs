use tracing::info;

#[tracing::instrument(skip(_input))]
pub fn process(_input: &str) -> miette::Result<String> {
    info!("{} - PART 2", "day01".to_uppercase());
    Ok("RESULT".into())
}

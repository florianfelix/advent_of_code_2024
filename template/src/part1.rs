use tracing::info;

#[tracing::instrument(skip(_input))]
pub fn process(_input: &str) -> miette::Result<String> {
    info!("{} - PART 1", "{{crate_name}}".to_uppercase());
    Ok("RESULT".into())
}

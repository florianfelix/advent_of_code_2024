pub fn trace() {
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        // .with_target(true)
        // .with_thread_ids(false)
        // .with_thread_names(false)
        // .with_file(true)
        // .with_line_number(true)
        .with_source_location(false)
        .with_ansi(true)
        .without_time()
        .compact();

    tracing_subscriber::fmt().event_format(format).init();
}

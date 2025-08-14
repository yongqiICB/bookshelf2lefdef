pub fn init_logger() {
    let logger = flexi_logger::Logger::try_with_env_or_str("info")
        .unwrap()
        .use_utc()
        .format_for_stderr(flexi_logger::colored_opt_format);
    logger.start().unwrap();
}
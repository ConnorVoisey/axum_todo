use super::Config;
use crate::config::get_env;
use std::io;
use tracing::metadata::LevelFilter;
use tracing_appender::rolling;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{Layer, Registry};

pub enum LogOutput {
    Console,
    File,
}

pub fn get_log_level(log_output: LogOutput) -> LevelFilter {
    let env_key = match log_output {
        LogOutput::Console => "LOG_CONSOLE_LEVEL",
        LogOutput::File => "LOG_FILE_NAME",
    };
    let val = get_env(env_key, Some(String::from("TRACE")));

    match val.as_str() {
        "TRACE" => LevelFilter::TRACE,
        "DEBUG" => LevelFilter::DEBUG,
        "INFO" => LevelFilter::INFO,
        "WARN" => LevelFilter::WARN,
        "ERROR" => LevelFilter::ERROR,
        "OFF" => LevelFilter::OFF,
        _ => panic!("invalid log level {val}, expected one of ['TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR', 'OFF']"),
    }
}

pub fn init_tracing(config: &Config) {
    let info_file = rolling::daily(&config.public.log_dir, &config.public.log_file_name);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(info_file)
        .with_ansi(false)
        .pretty()
        .with_level(true)
        .with_line_number(true)
        .with_file(true)
        .with_filter(config.public.log_file_level);

    let std_out_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(false)
        .pretty()
        .with_level(true)
        .with_line_number(true)
        .with_file(true)
        .with_filter(config.public.log_console_level);

    let subscriber = Registry::default().with(file_layer).with(std_out_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to setup tracing");
}

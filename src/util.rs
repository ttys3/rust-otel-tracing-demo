use std::fmt;
use env_logger::{Builder as LogBuilder, Target as LogTarget};
use log::LevelFilter;
use std::io::Write;

pub fn init_env_logger() {


    // https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html
    LogBuilder::new()
        .format(|buf, record| {

            let format = time::format_description::parse(
                "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
             sign:mandatory]:[offset_minute]:[offset_second]",
            ).unwrap();

            let local_time = time::OffsetDateTime::now_local().unwrap().format(&format).unwrap_or("".to_string());

            writeln!(
                buf,
                "{} [{}] - {}",
                local_time,
                buf.default_styled_level(record.level()),
                record.args()
            )
        })
        .filter_level(LevelFilter::Info) // set default level
        .parse_default_env() // then, if exists, respect the env config
        .target(LogTarget::Stdout)
        .init();
}


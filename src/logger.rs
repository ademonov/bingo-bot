use log::LogLevelFilter;
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn init()
{
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S[%Z])} [{l}] (({file}:{line})) - {m}{n}")))
        .build();

    let config = 
        Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("common", LogLevelFilter::Trace))
        .build(Root::builder().appender("stdout").build(LogLevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

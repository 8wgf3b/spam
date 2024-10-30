use dotenv_codegen::dotenv;
use spam::Daybreak;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;

fn main() {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "spam.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);
    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();
    info!("Look ma, I'm tracing!");
    let start_date = dotenv!("DATE");
    let msg = dotenv!("MSG");
    let confstr = include_str!("artifacts/config.txt");
    let token = dotenv!("GTOKEN");
    let db = Daybreak::new(start_date, msg, confstr, token);
    info!("Finished building daybreak");
    //println!("{}", db.checkdate("2024.10.28"));
    //db.burnice();
    db.simulate();
}

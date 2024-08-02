use anyhow::Result;
use clap::Parser;
use fern;
use httprs::core::httprs::HttpRs;
use std::{io, time::SystemTime};

/*
    TODO: Things to add  ->:
        paths of dirs which have a space in their name does not work
          Other Methods Support
          more than 1024 bytes for buffer of the request or even a dynamic allocator based on the buffer
          see if u can write TcpListener uself
          and TcpStream


*/

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value = "8080")]
    port: String,

    #[arg(short, long, default_value_t = 0)]
    verbose: u64,

    #[arg(short, long, default_value_t = false)]
    log: bool,

    #[arg(short, long, default_value = ".")]
    path: String,

    #[arg(short, long, default_value_t = false)]
    no_index_html: bool,
}

fn setup_logging(verbosity: u64, log: bool) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config
            .level(log::LevelFilter::Info)
            .level_for("overly-verbose-target", log::LevelFilter::Warn),
        1 => base_config
            .level(log::LevelFilter::Debug)
            .level_for("overly-verbose-target", log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            if record.level() > log::LevelFilter::Info {
                out.finish(format_args!(
                    "DEBUG @ {}: {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "[{} {}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    message
                ))
            }
        })
        .chain(io::stdout());

    if log {
        let file_config = fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .chain(fern::log_file("out.log")?);

        base_config
            .chain(file_config)
            .chain(stdout_config)
            .apply()?;
    } else {
        base_config.chain(stdout_config).apply()?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let host: &str = &args.host;
    let port: &str = &args.port;

    setup_logging(args.verbose, args.log).expect("Failed To Initialzie logger");

    let server: HttpRs = HttpRs::new(host, port, args.path.as_str(), args.no_index_html);

    server.serve()?;

    Ok(())
}

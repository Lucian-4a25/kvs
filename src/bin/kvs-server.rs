use std::{env, fmt, net::IpAddr, process, str::FromStr};

use clap::{App, AppSettings, Arg};
use kvs::{init_logger, validate_addr, Result};
use slog::error;

fn main() -> Result<()> {
    // eprintln!("Program arguments:");
    // for (index, argument) in env::args().enumerate() {
    //     eprintln!("  Argument {}: {}", index, argument);
    // }
    // eprintln!("End of arguments");
    let logger = init_logger();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        // .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("ADDR")
                .long("addr")
                .value_name("IP:PORT")
                .help("Sets the IP address and port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ENGINE")
                .long("engine")
                .value_name("ENGIN")
                .help("Specify the engine to use")
                .takes_value(true),
        )
        .get_matches();

    eprintln!("kvs-server {}:\n", env!("CARGO_PKG_VERSION"));
    let addr_value = matches.value_of("ADDR").unwrap_or("127.0.0.1:4000");
    let (ip_addr, port) = match validate_addr(addr_value) {
        Ok(result) => result,
        Err(error_message) => {
            error!(logger, "解析地址失败");
            panic!("{}", error_message);
        }
    };
    let engine_name = matches.value_of("ENGINE").unwrap_or("kvs");
    let engine: Engine = match engine_name.parse() {
        Ok(e) => e,
        Err(e) => {
            error!(logger, "Invalid engine name: {}", engine_name);
            panic!("Invalid engine name: {}", engine_name);
        }
    };

    // 现在 ip_addr 和 port 可以在这里使用
    // eprintln!("IP Address: {}, Port: {}", ip_addr, port);
    error!(logger, "Listening in: {}", addr_value);

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Engine {
    Kvs,
    Sled,
}

impl FromStr for Engine {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Engine, Self::Err> {
        match s {
            "kvs" => Ok(Engine::Kvs),
            "sled" => Ok(Engine::Sled),
            _ => Err(format!(
                "'{}' is not a valid engine. Use 'kvs' or 'sled'.",
                s
            )),
        }
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let engine_str = match self {
            Engine::Kvs => "kvs",
            Engine::Sled => "sled",
        };
        write!(f, "{}", engine_str)
    }
}

/// 启动服务
fn start_service(ip_addr: IpAddr, port: u16, engine: Engine) {}

use clap::{App, AppSettings, Arg};
use kvs::{validate_addr, Result, LOGGER};
use slog::{error, info, Logger};
use std::io::{Read, Write};
use std::{
    env, fmt,
    net::{IpAddr, SocketAddr, TcpListener},
    str::FromStr,
};

fn main() -> Result<()> {
    // eprintln!("Program arguments:");
    // for (index, argument) in env::args().enumerate() {
    //     eprintln!("  Argument {}: {}", index, argument);
    // }
    // eprintln!("End of arguments");

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

    error!(LOGGER, "kvs-server {}:\n", env!("CARGO_PKG_VERSION"));
    let addr_value = matches.value_of("ADDR").unwrap_or("127.0.0.1:4000");
    let (ip_addr, port) = match validate_addr(addr_value) {
        Ok(result) => result,
        Err(error_message) => {
            error!(LOGGER, "解析地址失败");
            panic!("{}", error_message);
        }
    };
    let engine_name = matches.value_of("ENGINE").unwrap_or("kvs");
    let engine: Engine = match engine_name.parse() {
        Ok(e) => e,
        Err(e) => {
            error!(LOGGER, "Invalid engine name: {}", engine_name);
            panic!("Invalid engine name: {}", engine_name);
        }
    };

    // 现在 ip_addr 和 port 可以在这里使用
    // eprintln!("IP Address: {}, Port: {}", ip_addr, port);
    error!(LOGGER, "Listening in: {}", addr_value);

    start_service(ip_addr, port, engine)
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
fn start_service(ip_addr: IpAddr, port: u16, engine: Engine) -> Result<()> {
    // 使用 ip_addr 和 port 构建 SocketAddr
    let socket_addr = SocketAddr::new(ip_addr, port);
    let listener = TcpListener::bind(socket_addr)?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        // 读取客户端发送的请求数据
        let mut buffer = [0; 512];
        stream.read(&mut buffer)?;
        info!(
            LOGGER,
            "Received from kvs-client: {}",
            String::from_utf8_lossy(&buffer[..])
        );

        // 处理请求并发送响应
        stream.write(b"Response from kvs-server")?;
    }

    Ok(())
}

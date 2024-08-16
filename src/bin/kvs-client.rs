use std::io::Write;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpStream;

use clap::{App, AppSettings, Arg, SubCommand};
use kvs::validate_addr;
use kvs::Result;
use kvs::LOGGER;
use serde::Deserialize;
use serde::Serialize;
use slog::info;
use slog::Logger;
use std::io::Read;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                )
                .arg(
                    Arg::with_name("ADDR")
                        .long("addr")
                        .value_name("IP:PORT")
                        .help("Sets the IP address and port")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("ADDR")
                        .long("addr")
                        .value_name("IP:PORT")
                        .help("Sets the IP address and port")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("ADDR")
                        .long("addr")
                        .value_name("IP:PORT")
                        .help("Sets the IP address and port")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let value = matches.value_of("VALUE").unwrap();
            let addr_value = matches.value_of("ADDR").unwrap_or("127.0.0.1:4000");
            let (ip_addr, port) = match validate_addr(addr_value) {
                Ok(result) => result,
                Err(error_message) => panic!("{}", error_message),
            };

            // 现在 ip_addr 和 port 可以在这里使用
            // println!("IP Address: {}, Port: {}", ip_addr, port);
            info!(LOGGER, "kvs-client listening in: {}", addr_value);
            let _ = send_to_server(
                ip_addr,
                port,
                ClientCommand::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                },
            );
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let addr_value = matches.value_of("ADDR").unwrap_or("127.0.0.1:4000");
            let (ip_addr, port) = match validate_addr(addr_value) {
                Ok(result) => result,
                Err(error_message) => panic!("{}", error_message),
            };

            // 现在 ip_addr 和 port 可以在这里使用
            // println!("IP Address: {}, Port: {}", ip_addr, port);
            info!(LOGGER, "kvs-client listening in: {}", addr_value);

            let _ = send_to_server(
                ip_addr,
                port,
                ClientCommand::Get {
                    key: key.to_string(),
                },
            );
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let addr_value = matches.value_of("ADDR").unwrap_or("127.0.0.1:4000");
            let (ip_addr, port) = match validate_addr(addr_value) {
                Ok(result) => result,
                Err(error_message) => panic!("{}", error_message),
            };

            // 现在 ip_addr 和 port 可以在这里使用
            // println!("IP Address: {}, Port: {}", ip_addr, port);
            info!(LOGGER, "kvs-client listening in: {}", addr_value);

            let _ = send_to_server(
                ip_addr,
                port,
                ClientCommand::Remove {
                    key: key.to_string(),
                },
            );
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

pub enum ServerResponse {
    Empty,
}

/// 向服务器发送消息
fn send_to_server(ip_addr: IpAddr, port: u16, command: ClientCommand) -> Result<ServerResponse> {
    // 使用 ip_addr 和 port 构建 SocketAddr
    let socket_addr = SocketAddr::new(ip_addr, port);
    let mut stream = TcpStream::connect(socket_addr)?;

    // 将 ClientCommand 对象序列化为 JSON 格式
    let serialized_command = serde_json::to_string(&command)?;

    stream.write_all(serialized_command.as_bytes())?;

    // 读取服务器的响应
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    // 将响应转换为字符串并返回
    let response = String::from_utf8(buffer)?;
    info!(LOGGER, "the response from server: {}", response);

    Ok(ServerResponse::Empty)
}

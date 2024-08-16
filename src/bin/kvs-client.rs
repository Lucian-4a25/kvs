use clap::{App, AppSettings, Arg, SubCommand};
use kvs::init_logger;
use kvs::validate_addr;
use kvs::Result;
use slog::info;

fn main() -> Result<()> {
    let logger = init_logger();

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
            info!(logger, "kvs-client listening in: {}", addr_value);
            // do the thing you need to do..
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
            info!(logger, "kvs-client listening in: {}", addr_value);

            // do the thing you need to do..
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
            info!(logger, "kvs-client listening in: {}", addr_value);
            // do the thing you need to do..
        }
        _ => unreachable!(),
    }

    Ok(())
}

use std::{env, process};

use clap::{App, AppSettings, Arg};
use kvs::{validate_addr, Result};

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

    eprintln!("kvs-server {}:\n", env!("CARGO_PKG_VERSION"));
    let addr_value = matches.value_of("ADDR").unwrap_or("127.0.0.1:4000");
    let (ip_addr, port) = match validate_addr(addr_value) {
        Ok(result) => result,
        Err(error_message) => {
            eprint!("解析地址失败");
            panic!("{}", error_message);
        }
    };

    // 现在 ip_addr 和 port 可以在这里使用
    // eprintln!("IP Address: {}, Port: {}", ip_addr, port);
    eprintln!("Listening in: {}", addr_value);

    Ok(())
}

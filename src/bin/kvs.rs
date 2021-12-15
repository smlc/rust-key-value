use clap::{App, Arg, SubCommand, AppSettings};
use kvs::KvStore;


fn main() {

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets key value in the store")
                .arg(Arg::with_name("key").required(true))
                .arg(Arg::with_name("value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get value for the given key")
                .arg(Arg::with_name("key").required(true))
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Delete entry for the given key")
                .arg(Arg::with_name("key").required(true))
        )
    .get_matches();
    
    let mut kvs = KvStore::new();
    if let Some(matches) = matches.subcommand_matches("set") {
        kvs.set(matches.value_of("key").unwrap().to_string(), matches.value_of("value").unwrap().to_string());
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        kvs.get(matches.value_of("key").unwrap().to_string());
    }

    if let Some(matches) = matches.subcommand_matches("rm") {
        kvs.remove(matches.value_of("key").unwrap().to_string());
    }
}
    
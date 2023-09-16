use clap::{value_parser, Arg, ArgMatches, Command};
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

#[tokio::main]
async fn main() {
    let cmd = getcmd();

    let port = match cmd.get_one::<u16>("port") {
        Some(r) => r,
        None => panic!("1"),
    };

    let ipaddr = match cmd.get_one::<String>("ipaddr") {
        Some(r) => r,
        None => "0.0.0.0",
    };

    let root = match cmd.get_one::<String>("root") {
        Some(r) => r,
        None => ".",
    };

    tracing_subscriber::fmt().init();

    let router = Router::with_path("<**path>")
        .get(StaticDir::new([root]).defaults("index.html").listing(true));

    let acceptor = TcpListener::new(format!("{}:{}", ipaddr, port))
        .bind()
        .await;
    Server::new(acceptor).serve(router).await;
}

fn getcmd() -> ArgMatches {
    let matches = Command::new("rshttpserver")
        .version("1.0")
        .author("Your Name")
        .about("A simple command line program")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .default_value("8075")
                .help("set port")
                .value_parser(value_parser!(u16)),
        )
        .arg(
            Arg::new("ipaddr")
                .short('i')
                .long("listen")
                .default_value("0.0.0.0")
                .help("set listen"),
        )
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .default_value(".")
                .help("set root path"),
        )
        .get_matches();
    matches
}

use anyhow::Context;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

mod p0;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let bind: SocketAddr = args
        .next()
        .expect("arg 1 missing: must provide bind addr:port")
        .parse()
        .context("invalid addr:port")?;
    let prob = args
        .next()
        .context("arg 2 missing: must provide problem idx")?;

    match prob.as_str() {
        "0" => run(bind, p0::p0),
        bogus => anyhow::bail!("invalid problem idx: {bogus}"),
    }?;

    Ok(())
}

fn run(
    bind: SocketAddr,
    f: fn(TcpStream) -> anyhow::Result<()>,
) -> anyhow::Result<Infallible, std::io::Error> {
    let srv = TcpListener::bind(bind)?;
    loop {
        let (stream, addr) = srv.accept()?;
        println!("connection from {addr}");
        std::thread::spawn(move || {
            if let Err(err) = f(stream) {
                println!("{:#?}", err);
                std::process::exit(1);
            }
            println!("done responding to {addr}!");
        });
    }
}

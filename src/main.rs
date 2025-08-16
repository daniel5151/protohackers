use anyhow::Context;
use std::convert::Infallible;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

mod p0;
mod p1;

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
        "1" => run(bind, p1::p1),
        bogus => anyhow::bail!("invalid problem idx: {bogus}"),
    }?;

    Ok(())
}

fn run(
    bind: SocketAddr,
    f: fn(TcpStream, TcpStream) -> anyhow::Result<bool>,
) -> anyhow::Result<Infallible, std::io::Error> {
    let srv = TcpListener::bind(bind)?;
    loop {
        let (rx, addr) = srv.accept()?;
        rx.set_nodelay(false)?;
        let tx = rx.try_clone()?;
        let mut mal = rx.try_clone()?;
        println!("connection from {addr}");
        std::thread::spawn(move || {
            let mut handle = |res| {
                println!("done responding to {addr}!");
                match res {
                    Ok(true) => anyhow::Ok(()),
                    Ok(false) => Ok(mal.write_all(b"{}")?),
                    Err(e) => Err(e),
                }
            };

            if let Err(e) = handle(f(tx, rx)) {
                println!("{:#?}", e);
                std::process::exit(1);
            }
        });
    }
}

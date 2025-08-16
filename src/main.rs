use anyhow::Context;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

mod p0;
mod p1;
mod p2;
mod p3;
mod util;

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
        "0" => run::<p0::P0>(bind),
        "1" => run::<p1::P1>(bind),
        "2" => run::<p2::P2>(bind),
        "3" => run::<p3::P3>(bind),
        bogus => anyhow::bail!("invalid problem idx: {bogus}"),
    }?;

    Ok(())
}

trait Server: Send + Sync + 'static {
    fn init() -> Self;
    fn accept(&self, addr: SocketAddr, tx: TcpStream, rx: TcpStream) -> anyhow::Result<()>;
}

fn run<T: Server>(bind: SocketAddr) -> anyhow::Result<Infallible, std::io::Error> {
    let listener = TcpListener::bind(bind)?;
    let server = Arc::new(T::init());
    loop {
        let (rx, addr) = listener.accept()?;
        let tx = rx.try_clone()?;
        let server = server.clone();
        println!("[{addr}] connected");
        std::thread::spawn(move || {
            if let Err(e) = server.accept(addr, tx, rx) {
                println!("{:#?}", e);
                std::process::exit(1);
            }
            println!("[{addr}] disconnected");
        });
    }
}

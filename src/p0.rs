use std::net::TcpStream;

pub struct P0;

impl crate::Server for P0 {
    fn init() -> Self {
        Self
    }

    fn accept(
        &self,
        _addr: std::net::SocketAddr,
        mut tx: TcpStream,
        mut rx: TcpStream,
    ) -> anyhow::Result<()> {
        std::io::copy(&mut rx, &mut tx)?;
        Ok(())
    }
}

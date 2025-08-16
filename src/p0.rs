use std::net::TcpStream;

pub fn p0(mut tx: TcpStream, mut rx: TcpStream) -> anyhow::Result<bool> {
    std::io::copy(&mut rx, &mut tx)?;
    Ok(true)
}

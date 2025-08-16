use std::net::TcpStream;

pub fn p0(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut backstream = stream.try_clone().unwrap();
    std::io::copy(&mut stream, &mut backstream)?;
    Ok(())
}

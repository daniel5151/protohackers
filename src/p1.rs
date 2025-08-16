use serde::Deserialize;
use serde::Serialize;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

#[derive(Deserialize)]
struct Request {
    method: String,
    number: f64,
}

#[derive(Serialize)]
struct Response {
    method: &'static str,
    prime: bool,
}

pub struct P1;

impl crate::Server for P1 {
    fn init() -> Self {
        Self
    }

    fn accept(
        &self,
        addr: std::net::SocketAddr,
        mut tx: TcpStream,
        rx: TcpStream,
    ) -> anyhow::Result<()> {
        for line in BufReader::new(rx).lines() {
            let line = line?;
            println!("[{addr}] {line}");

            let Ok(req) = serde_json::from_str::<Request>(&line) else {
                tx.write_all(b"x")?;
                return Ok(());
            };

            if req.method != "isPrime" {
                tx.write_all(b"x")?;
                return Ok(());
            }

            serde_json::to_writer(
                &mut tx,
                &Response {
                    method: "isPrime",
                    prime: is_prime(req.number),
                },
            )?;
            tx.write_all(b"\n")?;
        }

        Ok(())
    }
}

// thanks chatgpt
fn is_prime(n: f64) -> bool {
    if !(n.is_finite() && n.fract() == 0.0 && n > 0.0 && n < u64::MAX as f64) {
        return false;
    }

    let n = n as u64;

    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

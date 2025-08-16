use crate::util::be_types::*;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use zerocopy::FromBytes;
use zerocopy::FromZeros;
use zerocopy::IntoBytes;

#[derive(FromBytes, IntoBytes)]
#[repr(C, packed)]
struct Msg {
    ty: u8,
    args: [i32_be; 2],
}

// NOTE: the 200k request test is pretty tricky when done on a residential
// network, with port forwarding, in WSL2... 60s isn't always enough.
pub fn p2(mut tx: TcpStream, rx: TcpStream) -> anyhow::Result<bool> {
    let mut prices = BTreeMap::new();

    let mut rx = BufReader::new(rx);
    let mut msg = Msg::new_zeroed();
    let mut count = 0;
    loop {
        if let Err(e) = rx.read_exact(msg.as_mut_bytes()) {
            if matches!(e.kind(), std::io::ErrorKind::UnexpectedEof) {
                break;
            }
            return Err(e.into());
        }
        std::io::stdout().write_fmt(format_args!(
            "in {count}: {}: {:?}\n",
            msg.ty as char,
            msg.args.map(|x| x.get())
        ))?;

        match (msg.ty, msg.args.map(|x| x.get())) {
            (b'I', [timestamp, price]) => {
                prices.insert(timestamp, price);
                count += 1;
            }
            (b'Q', [mintime, maxtime]) => {
                let ret = if maxtime >= mintime {
                    let (sum, count): (i64, _) = prices
                        .range(mintime..=maxtime)
                        .fold((0, 0), |(sum, count), (_, v)| (sum + *v as i64, count + 1));
                    if count == 0 { 0 } else { sum / count }
                } else {
                    0
                };
                tx.write_all(i32_be::new(ret as i32).as_bytes())?;
            }
            _ => return Ok(false),
        }
    }

    Ok(true)
}

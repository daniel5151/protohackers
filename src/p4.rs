use std::collections::HashMap;

pub struct P4;

impl crate::UdpServer for P4 {
    fn init() -> Self {
        Self
    }

    // NOTE: UDP on a residential network, with port forwarding, in WSL2...
    // really sucks. To pass this test - I spun up a free VM on Azure for ~5
    // mins, and just ran the server on there.
    fn run(self, sock: std::net::UdpSocket) -> anyhow::Result<std::convert::Infallible> {
        let mut store = HashMap::new();
        store.insert(b"version".into(), b"1337".into());

        let mut buf = [0; 1000];
        loop {
            let (n, addr) = sock.recv_from(&mut buf)?;
            let msg = &buf[..n];
            println!("[{addr}] --> {}", String::from_utf8_lossy(msg));

            match msg
                .iter()
                .position(|x| *x == b'=')
                .map(|pos| msg.split_at(pos))
            {
                Some((b"version", _)) => {} // don't modify version
                Some((k, v)) => {
                    store.insert(k.to_vec(), v[1..].to_vec());
                }
                None => {
                    let val = store.get(msg).map(|x| x.as_slice()).unwrap_or_default();
                    buf[n..][0] = b'=';
                    buf[n + 1..][..val.len()].copy_from_slice(val);
                    let buf = &buf[..n + 1 + val.len()];
                    println!("[{addr}] <-- {}", String::from_utf8_lossy(buf));
                    sock.send_to(buf, addr)?;
                }
            }
        }
    }
}

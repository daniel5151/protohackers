use parking_lot::Mutex;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

struct Peer {
    name: String,
    tx: Mutex<TcpStream>,
}

pub struct P3 {
    uid: AtomicUsize,
    peers: RwLock<HashMap<usize, Peer>>,
}

impl crate::Server for P3 {
    fn init() -> Self {
        Self {
            uid: AtomicUsize::new(0),
            peers: RwLock::new(HashMap::new()),
        }
    }

    fn accept(
        &self,
        addr: std::net::SocketAddr,
        mut tx: TcpStream,
        rx: TcpStream,
    ) -> anyhow::Result<()> {
        let mut rx = BufReader::new(rx);
        writeln!(tx, "Welcome to budgetchat! What's your name?")?;
        let mut name = String::new();
        rx.read_line(&mut name)?;
        name.pop(); // strip trailing newline

        if name.is_empty() || name.as_bytes().iter().any(|b| !b.is_ascii_alphanumeric()) {
            println!("[{addr}] invalid name: {name}");
            write!(tx, "invalid name")?;
            return Ok(());
        }

        let id = self.uid.fetch_add(1, Ordering::Relaxed);
        println!("[{addr}] added {id}:{name}");

        write!(tx, "* The room contains:")?;
        for (i, Peer { name, .. }) in self.peers.read().values().enumerate() {
            if i != 0 {
                write!(tx, ",")?;
            }
            write!(tx, " {name}")?;
        }
        writeln!(tx)?;

        // notify peers of connect
        for Peer { tx, .. } in self.peers.read().values() {
            let mut tx = tx.lock();
            // ignore err, in case peer drops mid-notify
            let _ = writeln!(tx, "* {name} has entered the room");
        }

        // only add new peer after sending room contains message
        self.peers.write().insert(
            id,
            Peer {
                name: name.clone(),
                tx: Mutex::new(tx),
            },
        );

        let mut msg = String::new();
        loop {
            msg.clear();
            if matches!(rx.read_line(&mut msg), Ok(0)) {
                break;
            }
            msg.pop(); // strip trailing newline

            println!("[{addr}] {id}:{name} --> {msg}");

            // notify peers of message
            for (other_id, Peer { tx, .. }) in self.peers.read().iter() {
                // don't send message back to self
                if id == *other_id {
                    continue;
                }

                let mut tx = tx.lock();
                // ignore err, in case peer drops mid-notify
                let _ = writeln!(tx, "[{name}] {msg}");
            }
        }

        self.peers.write().remove(&id);
        println!("[{addr}] removed {id}:{name}");

        // notify remaining peers of disconnect
        for Peer { tx, .. } in self.peers.read().values() {
            let mut tx = tx.lock();
            // ignore err, in case peer drops mid-notify
            let _ = writeln!(tx, "* {name} has left the room");
        }

        Ok(())
    }
}

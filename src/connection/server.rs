use crate::lsmtree::LSMTree;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        Self { port }
    }

    pub fn start(&self, lsm: Arc<Mutex<LSMTree>>) -> std::io::Result<()> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr)?;
        println!("Listening on {addr}");

        for stream in listener.incoming() {
            let tree = Arc::clone(&lsm);
            std::thread::spawn(move || {
                if let Ok(stream) = stream {
                    Self::handle_client(stream, tree);
                }
            });
        }

        Ok(())
    }

    fn handle_client(stream: TcpStream, tree: Arc<Mutex<LSMTree>>) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = stream;

        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line).unwrap_or(0);
            if n == 0 {
                break; // connection closed
            }

            let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();

            if parts.is_empty() {
                continue;
            }
            println!("Received command: {line}");

            let response = match parts[0].to_uppercase().as_str() {
                "GET" if parts.len() == 2 => {
                    let val = tree.lock().unwrap().get(parts[1]);
                    match val {
                        Some(v) => format!("OK {}\n", v),
                        None => "NOTFOUND\n".to_string(),
                    }
                }
                "PUT" if parts.len() == 3 => {
                    tree.lock()
                        .unwrap()
                        .put(parts[1].to_string(), parts[2].to_string());
                    "OK\n".to_string()
                }
                "DEL" if parts.len() == 2 => {
                    tree.lock().unwrap().delete(parts[1].to_string());
                    "OK\n".to_string()
                }
                _ => "ERR Invalid command\n".to_string(),
            };

            writer.write_all(response.as_bytes()).ok();
            writer.flush().ok();
        }
    }
}

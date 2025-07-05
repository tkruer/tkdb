use std::sync::{Arc, Mutex};
use tkdb::{connection::server::Server, db};

fn main() -> std::io::Result<()> {
    let db = db::TKDB::new()
        .with_sstable_dir("data")
        .with_threshold(5)
        .build()?;

    let server = Server::new(4000);
    server.start(Arc::new(Mutex::new(db)))?;
    Ok(())
}

use crate::tkdb::*;

use std::path::PathBuf;

struct Args {
    #[arg(long, default_value = "tkdb.db")]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let db = DB::new(args.path, DBOptions {
        path: args.path
    });
    Ok(())
}

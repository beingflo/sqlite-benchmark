use crate::scenarios::{simple::simple, wal::wal, wal_synchronous::wal_synchronous};

mod migration;
mod scenarios;
mod stats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Single thread: ");
    simple()?;
    print!("Single thread + WAL mode: ");
    wal()?;
    print!("Single thread + WAL mode + synchronous normal: ");
    wal_synchronous()?;

    Ok(())
}

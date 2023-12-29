use crate::scenarios::{
    simple::simple, single_mutex::single_mutex, wal::wal, wal_synchronous::wal_synchronous,
    wal_synchronous_memory::wal_synchronous_memory,
};

mod migration;
mod scenarios;
mod stats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Single thread + Mutex: ");
    single_mutex()?;
    print!("Single thread + WAL mode + synchronous normal + in-memory: ");
    wal_synchronous_memory()?;
    print!("Single thread + WAL mode + synchronous normal: ");
    wal_synchronous()?;
    print!("Single thread + WAL mode: ");
    wal()?;
    print!("Single thread: ");
    simple()?;

    Ok(())
}

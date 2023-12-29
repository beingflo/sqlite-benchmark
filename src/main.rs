use std::fs;

use crate::scenarios::{
    index::index, simple::simple, simple_read::simple_read, single_mutex::single_mutex, wal::wal,
    wal_read::wal_read, wal_synchronous::wal_synchronous,
    wal_synchronous_memory::wal_synchronous_memory, wal_synchronous_read::wal_synchronous_read,
};

mod migration;
mod scenarios;
mod stats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Single Thread
    print!("Single thread + WAL mode + synchronous normal: ");
    wal_synchronous()?;
    print!("Single thread + WAL mode + synchronous normal read: ");
    wal_synchronous_read()?;
    fs::remove_file("./wal-synchronous.sqlite")?;

    return Ok(());

    print!("Single thread + WAL mode: ");
    wal()?;
    print!("Single thread + WAL mode read: ");
    wal_read()?;
    fs::remove_file("./wal.sqlite")?;

    print!("Single thread: ");
    simple()?;
    print!("Single thread read: ");
    simple_read()?;
    fs::remove_file("./simple.sqlite")?;

    print!("Single thread + Index: ");
    index()?;
    print!("Single thread + Mutex: ");
    single_mutex()?;
    print!("Single thread + WAL mode + synchronous normal + in-memory: ");
    wal_synchronous_memory()?;
    Ok(())
}

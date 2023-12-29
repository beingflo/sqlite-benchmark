use std::fs;

use crate::scenarios::{
    index::index, simple::simple, simple_read::simple_read, single_mutex::single_mutex, wal::wal,
    wal_synchronous::wal_synchronous, wal_synchronous_memory::wal_synchronous_memory,
};

mod migration;
mod scenarios;
mod stats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Single thread: ");
    simple()?;
    print!("Single thread read: ");
    simple_read()?;
    fs::remove_file("./simple.sqlite")?;

    return Ok(());

    // Single Thread
    // WRITE
    print!("Single thread + Index: ");
    index()?;
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
    // READ

    Ok(())
}

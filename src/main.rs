use std::fs;

use crate::scenarios::{
    index::index, index_read::index_read, simple::simple, simple_read::simple_read,
    single_mutex::single_mutex, wal::wal, wal_read::wal_read, wal_synchronous::wal_synchronous,
    wal_synchronous_memory::wal_synchronous_memory,
    wal_synchronous_memory_read::wal_synchronous_memory_read,
    wal_synchronous_read::wal_synchronous_read,
};

mod migration;
mod scenarios;
mod stats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Single Thread
    print!("Single thread + Index: ");
    index()?;
    print!("Single thread + Index read: ");
    index_read()?;
    fs::remove_file("./index.sqlite")?;

    print!("Single thread + WAL mode + synchronous normal + in-memory: ");
    let conn = wal_synchronous_memory()?;
    print!("Single thread + WAL mode + synchronous normal + in-memory read: ");
    wal_synchronous_memory_read(conn)?;

    print!("Single thread + WAL mode + synchronous normal: ");
    wal_synchronous()?;
    print!("Single thread + WAL mode + synchronous normal read: ");
    wal_synchronous_read()?;
    fs::remove_file("./wal-synchronous.sqlite")?;

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

    print!("Single thread + Mutex: ");
    single_mutex()?;

    Ok(())
}

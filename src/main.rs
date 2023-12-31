use std::fs;

use crate::scenarios::{
    index::index, index_mixed::index_mixed, index_read::index_read, multi::multi, simple::simple,
    simple_read::simple_read, single_mutex::single_mutex, wal::wal, wal_read::wal_read,
    wal_synchronous::wal_synchronous, wal_synchronous_memory::wal_synchronous_memory,
    wal_synchronous_memory_read::wal_synchronous_memory_read,
    wal_synchronous_multi::wal_synchronous_multi, wal_synchronous_read::wal_synchronous_read,
};

mod migration;
mod scenarios;
mod stats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Single thread + WAL mode + synchronous normal: ");
    wal_synchronous()?;
    print!("Multi thread + WAL mode + synchronous normal: ");
    wal_synchronous_multi()?;
    fs::remove_file("./wal-synchronous.sqlite")?;

    // Single Thread
    print!("Single thread + Index mixed read heavy: ");
    index_mixed(false)?;
    fs::remove_file("./index_mixed.sqlite")?;
    print!("Single thread + Index mixed write heavy: ");
    index_mixed(true)?;
    fs::remove_file("./index_mixed.sqlite")?;

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
    fs::remove_file("./single_mutex.sqlite")?;

    Ok(())
}

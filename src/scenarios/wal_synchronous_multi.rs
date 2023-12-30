use std::thread;
use std::time::Instant;

use rusqlite::Connection;

use crate::migration::apply_migrations;
use crate::migration::Entry;

pub fn wal_synchronous_multi() -> Result<(), Box<dyn std::error::Error>> {
    let num_threads = 1;
    let mut handles = vec![];

    let before = Instant::now();

    (0..num_threads).for_each(|_| {
        let sender = thread::spawn(move || {
            let mut conn = Connection::open("./wal-synchronous.sqlite").unwrap();

            conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
                .unwrap();
            conn.pragma_update(None, "synchronous", &"NORMAL").unwrap();
            apply_migrations(&mut conn);

            let mut stmt = conn
                .prepare("SELECT * FROM metrics WHERE bucket = ?1 AND rowid = ?2;")
                .unwrap();

            let num_iterations = 2000000 / num_threads;

            let mut counter = 0;
            while counter < num_iterations {
                let bucket: String = "test".into();

                let random = (rand::random::<u32>() % 20000) + 1;

                let rows = stmt
                    .query_and_then(
                        [bucket, random.to_string()],
                        |row| -> Result<Entry, rusqlite::Error> {
                            Ok(Entry {
                                bucket: row.get(0).unwrap(),
                                date: row.get(1).unwrap(),
                                data: row.get(2).unwrap(),
                            })
                        },
                    )
                    .unwrap();

                assert!(rows.count() >= 1);

                counter += 1;
            }
        });
        handles.push(sender);
    });

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    let after = Instant::now();

    let duration = after - before;
    println!("{}", duration.as_millis());

    Ok(())
}

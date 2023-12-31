use std::{
    thread,
    time::{Instant, SystemTime},
};

use chrono::{DateTime, Utc};
use r2d2_sqlite::SqliteConnectionManager;

use crate::migration::{apply_migrations_with_index, Entry};

pub fn multi(write_heavy: bool) -> Result<(), Box<dyn std::error::Error>> {
    let manager = SqliteConnectionManager::file("./multi.sqlite");
    let pool = r2d2::Pool::new(manager).unwrap();

    pool.get()
        .unwrap()
        .pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
        .unwrap();
    pool.get()
        .unwrap()
        .pragma_update(None, "synchronous", &"NORMAL")
        .unwrap();
    apply_migrations_with_index(&mut pool.get().unwrap());

    let num_threads = 8;
    let mut handles = vec![];

    let before = Instant::now();

    let num_iterations = 2000000 / num_threads;

    (0..num_threads).for_each(|_| {
        let pool = pool.clone();
        let sender = thread::spawn(move || {
            let conn = pool.get().unwrap();

            let mut stmt = conn
                .prepare("SELECT * FROM metrics WHERE bucket = ?1 AND rowid = ?2;")
                .unwrap();

            let mut counter = 0;
            while counter < num_iterations {
                let bucket: String = "test".into();
                counter += 1;

                let read = {
                    let random = rand::random::<f32>();
                    if write_heavy {
                        random < 1.0
                    } else {
                        random < 1.0
                    }
                };

                if read {
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
                } else {
                    let now = SystemTime::now();
                    let now: DateTime<Utc> = now.into();

                    let date = now.to_rfc3339();
                    let data: String = "data".into();

                    let result = conn
                        .execute(
                            "INSERT INTO metrics (bucket, date, data) VALUES (?1, ?2, ?3)",
                            (bucket, date, data),
                        )
                        .unwrap();
                    assert_eq!(result, 1);
                }
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

use std::time::{Instant, SystemTime};

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::{
    migration::{apply_migrations_with_index, Entry},
    stats::Measurements,
};

pub fn index_mixed(write_heavy: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open("./index_mixed.sqlite")?;

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
        .unwrap();
    conn.pragma_update(None, "synchronous", &"NORMAL").unwrap();
    apply_migrations_with_index(&mut conn);

    let mut measurements = Measurements::new();

    let num_iterations = 100000;

    let mut stmt = conn
        .prepare("SELECT * FROM metrics WHERE bucket = ?1 AND rowid = ?2;")
        .unwrap();

    let mut counter = 0;
    let mut inserts = 0;
    while counter < num_iterations {
        let bucket: String = "test".into();

        let read = {
            let random = rand::random::<f32>();
            if write_heavy {
                random < 0.2
            } else {
                random < 0.8
            }
        };

        let before;
        if read {
            if inserts < 10 {
                continue;
            }
            let random = (rand::random::<u32>() % inserts) + 1;

            before = Instant::now();

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
            inserts += 1;

            before = Instant::now();
            let result = conn
                .execute(
                    "INSERT INTO metrics (bucket, date, data) VALUES (?1, ?2, ?3)",
                    (bucket, date, data),
                )
                .unwrap();
            assert_eq!(result, 1);
        }

        let after = Instant::now();

        let duration = after - before;
        measurements.insert(duration.as_micros());

        counter += 1;
    }

    measurements.print_results();

    Ok(())
}

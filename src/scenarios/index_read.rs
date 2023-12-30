use std::time::Instant;

use rusqlite::Connection;

use crate::{
    migration::{apply_migrations_with_index, Entry},
    stats::Measurements,
};

pub fn index_read() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open("./index.sqlite")?;

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
        .unwrap();
    conn.pragma_update(None, "synchronous", &"NORMAL").unwrap();
    apply_migrations_with_index(&mut conn);

    let mut measurements = Measurements::new();

    let num_iterations = 1000;

    let mut stmt = conn
        .prepare("SELECT * FROM metrics WHERE bucket = ?1 AND rowid = ?2;")
        .unwrap();

    let mut counter = 0;
    while counter < num_iterations {
        let bucket: String = "test".into();
        let random = rand::random::<u32>() % 20000;

        let before = Instant::now();

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
        let after = Instant::now();

        let duration = after - before;
        measurements.insert(duration.as_micros());

        counter += 1;
    }

    measurements.print_results();

    Ok(())
}

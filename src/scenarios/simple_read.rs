use std::time::Instant;

use rusqlite::Connection;

use crate::{
    migration::{apply_migrations, Entry},
    stats::Measurements,
};

pub fn simple_read() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open("./simple.sqlite")?;

    apply_migrations(&mut conn);

    let mut measurements = Measurements::new();

    let num_iterations = 100;

    let mut stmt = conn
        .prepare("SELECT * FROM metrics WHERE bucket = ?1 ORDER BY date")
        .unwrap();

    let mut counter = 0;
    while counter < num_iterations {
        let bucket: String = "test".into();

        let before = Instant::now();

        let rows = stmt
            .query_and_then([bucket], |row| -> Result<Entry, rusqlite::Error> {
                Ok(Entry {
                    bucket: row.get(0).unwrap(),
                    date: row.get(1).unwrap(),
                    data: row.get(2).unwrap(),
                })
            })
            .unwrap();

        assert!(rows.count() >= 100);
        let after = Instant::now();

        let duration = after - before;
        measurements.insert(duration.as_micros());

        counter += 1;
    }

    measurements.print_results();

    Ok(())
}

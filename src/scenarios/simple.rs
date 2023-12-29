use std::time::{Instant, SystemTime};

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::{migration::apply_migrations, stats::Measurements};

pub fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open("./simple.sqlite")?;

    apply_migrations(&mut conn);

    let mut measurements = Measurements::new();

    let num_iterations = 20000;

    let mut counter = 0;
    while counter < num_iterations {
        let bucket: String = "test".into();

        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let date = now.to_rfc3339();
        let data: String = "data".into();

        let before = Instant::now();
        let result = conn
            .execute(
                "INSERT INTO metrics (bucket, date, data) VALUES (?1, ?2, ?3)",
                (bucket, date, data),
            )
            .unwrap();
        let after = Instant::now();

        let duration = after - before;
        measurements.insert(duration.as_micros());

        assert_eq!(result, 1);

        counter += 1;
    }

    measurements.print_results();

    Ok(())
}

use std::{
    sync::Mutex,
    time::{Instant, SystemTime},
};

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::{migration::apply_migrations, stats::Measurements};

pub fn single_mutex() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Mutex::new(Connection::open("./single_mutex.sqlite")?);

    {
        let c = conn.lock().unwrap();
        c.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
            .unwrap();
        c.pragma_update(None, "synchronous", &"NORMAL").unwrap();
    }
    apply_migrations(conn.get_mut().unwrap());

    let mut measurements = Measurements::new();

    let num_iterations = 100000;

    let mut counter = 0;
    while counter < num_iterations {
        let bucket: String = "test".into();

        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let date = now.to_rfc3339();
        let data: String = "data".into();

        let before = Instant::now();
        let result = conn
            .lock()
            .unwrap()
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

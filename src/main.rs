mod migration;
mod stats;

use migration::apply_migrations;
use rusqlite::Connection;
use stats::Measurements;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open("./db.sqlite")?;

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
        .unwrap();
    conn.pragma_update(None, "synchronous", &"NORMAL").unwrap();
    apply_migrations(&mut conn);

    let mut measurements = Measurements::new();

    let num_iterations = 10000;

    let mut counter = 0;
    while counter < num_iterations {
        let bucket: String = "test".into();
        let date: String = "date".into();
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

    println!(
        "avg: {}, 90%: {}, throughput: {:.2}",
        measurements.get_average(),
        measurements.get_90_percentile(),
        measurements.get_throughput()
    );

    Ok(())
}

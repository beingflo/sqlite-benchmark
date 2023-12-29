use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

pub fn apply_migrations(connection: &mut Connection) {
    let migrations = Migrations::new(vec![M::up(
        "CREATE TABLE metrics(bucket TEXT NOT NULL, date TEXT NOT NULL, data TEXT NOT NULL);",
    )]);

    migrations.to_latest(connection).unwrap();
}

pub fn apply_migrations_with_index(connection: &mut Connection) {
    let migrations = Migrations::new(vec![
        M::up(
            "CREATE TABLE metrics(bucket TEXT NOT NULL, date TEXT NOT NULL, data TEXT NOT NULL);",
        ),
        M::up(
            "CREATE INDEX bucket_index
            ON metrics(bucket);",
        ),
        M::up(
            "CREATE INDEX date_index
            ON metrics(date);",
        ),
    ]);

    migrations.to_latest(connection).unwrap();
}

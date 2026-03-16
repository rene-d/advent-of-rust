use std::{error::Error, path::PathBuf, time::Duration};

use rusqlite::{Connection, Result};
use sha2::{Digest, Sha256};

/// Trait for database backends that store puzzle execution timings.
pub trait TimingsDb {
    /// Updates the recorded execution time for a specific puzzle.
    ///
    /// It updates the record only if the new `elapsed` time is shorter than the one currently stored.
    /// Returns the best execution time recorded so far.
    ///
    /// # Errors
    /// Returns an error if the database operation fails or type conversion fails.
    fn update(
        &self,
        year: u16,
        day: u8,
        data: &str,
        elapsed: Duration,
    ) -> Result<Duration, Box<dyn Error>>;
}

/// SQLite-based implementation of execution timings database.
pub struct RunDb {
    conn: Connection,
}

impl RunDb {
    /// Creates a new `RunDb` instance and ensures the `timings` table exists in `cache.db`.
    ///
    /// # Errors
    /// Returns an error if the database file cannot be opened or the schema cannot be initialized.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .ok()
            .unwrap_or_else(|| ".".to_string());

        let path = PathBuf::from(manifest_dir).join("cache.db");

        let conn = Connection::open(path)?;

        conn.execute_batch(
            "create table if not exists timings (
                year integer not null,
                day integer not null,
                sha256 text not null,
                elapsed integer not null
            );
            create unique index if not exists idx_timings on timings (year,day,sha256);
            ",
        )?;

        Ok(Self { conn })
    }
}

impl TimingsDb for RunDb {
    /// # Errors
    fn update(
        &self,
        year: u16,
        day: u8,
        data: &str,
        elapsed: Duration,
    ) -> Result<Duration, Box<dyn Error>> {
        let mut hasher = Sha256::new();
        hasher.update(data.trim_ascii());
        let digest = format!("{:x}", hasher.finalize());

        let elapsed_micros = i64::try_from(elapsed.as_micros())?;

        let mut stmt = self
            .conn
            .prepare("select elapsed from timings where year=?1 and day=?2 and sha256=?3")?;

        let best_micros = if let Ok(previous_micros) = stmt
            .query_row((&year, &(u16::from(day)), &digest), |row| {
                row.get::<usize, i64>(0)
            }) {
            if elapsed_micros < previous_micros {
                self.conn.execute(
                    "update timings set elapsed=?4 where year=?1 and day=?2 and sha256=?3",
                    (&year, &(u16::from(day)), &digest, &elapsed_micros),
                )?;
                elapsed_micros
            } else {
                previous_micros
            }
        } else {
            self.conn.execute(
                "insert into timings (year,day,sha256,elapsed) values (?1, ?2, ?3, ?4)",
                (&year, &(u16::from(day)), &digest, &elapsed_micros),
            )?;
            elapsed_micros
        };

        Ok(Duration::from_micros(u64::try_from(best_micros)?))
    }
}

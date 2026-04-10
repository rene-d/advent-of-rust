use std::{error::Error, path::PathBuf, time::Duration};

use libsql::{Builder, Connection};
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

/// SQLite-based implementation of execution timings database using Turso (libsql).
pub struct RunDb {
    conn: Connection,
    rt: tokio::runtime::Runtime,
}

impl RunDb {
    /// Creates a new `RunDb` instance and ensures the `timings` table exists in `.timings.db`.
    ///
    /// # Errors
    /// Returns an error if the database file cannot be opened or the schema cannot be initialized.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .ok()
            .unwrap_or_else(|| ".".to_string());

        let path = PathBuf::from(manifest_dir).join(".timings.db");

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let db = rt.block_on(Builder::new_local(path).build())?;
        let conn = db.connect()?;

        rt.block_on(conn.execute_batch(
            "create table if not exists timings (
                year integer not null,
                day integer not null,
                crc text not null,
                elapsed_ns integer not null
            );
            create unique index if not exists idx_timings on timings (year,day,crc);
            ",
        ))?;

        Ok(Self { conn, rt })
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
        let digest = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        let elapsed_nanos = i64::try_from(elapsed.as_nanos())?;

        let best_nanos = self.rt.block_on(async {
            let mut rows = self
                .conn
                .query(
                    "select elapsed_ns from timings where year=?1 and day=?2 and crc=?3",
                    (i64::from(year), i64::from(day), digest.clone()),
                )
                .await?;

            if let Some(row) = rows.next().await? {
                let previous_micros: i64 = row.get(0)?;
                if elapsed_nanos < previous_micros {
                    self.conn
                        .execute(
                            "update timings set elapsed_ns=?4 where year=?1 and day=?2 and crc=?3",
                            (
                                i64::from(year),
                                i64::from(day),
                                digest.clone(),
                                elapsed_nanos,
                            ),
                        )
                        .await?;
                    Ok::<i64, Box<dyn Error>>(elapsed_nanos)
                } else {
                    Ok(previous_micros)
                }
            } else {
                self.conn
                    .execute(
                        "insert into timings (year,day,crc,elapsed_ns) values (?1, ?2, ?3, ?4)",
                        (i64::from(year), i64::from(day), digest, elapsed_nanos),
                    )
                    .await?;
                Ok(elapsed_nanos)
            }
        })?;

        Ok(Duration::from_nanos(u64::try_from(best_nanos)?))
    }
}

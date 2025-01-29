use std::{error::Error, path::PathBuf, time::Duration};

use rusqlite::{Connection, Result};
use sha2::{Digest, Sha256};

/// # Errors
pub fn update_db(year: u16, day: u8, data: &str, elapsed: Duration) -> Result<(), Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .ok()
        .unwrap_or_else(|| ".".to_string());

    let path = PathBuf::from(manifest_dir).join(".run.db");

    let conn = Connection::open(path)?;

    conn.execute_batch(
        "create table if not exists timings (
            year integer not null,
            day integer not null,
            sha256 text not null,
            elapsed number not null,
            count integer not null
        );
        create unique index if not exists idx_timings on timings (year,day,sha256);
        create view if not exists best as select year,day,min(elapsed) as elapsed from timings group by year,day;
        ",
    )?;

    let mut hasher = Sha256::new();
    hasher.update(data.trim_ascii());
    let result = hasher.finalize();

    let digest = format!("{result:x}");

    let elapsed = u64::try_from(elapsed.as_micros())?;

    let mut stmt =
        conn.prepare("select elapsed,count from timings where year=?1 and day=?2 and sha256=?3")?;

    if let Ok((previous_elapsed, count)) = stmt.query_row((&year, &day, &digest), |row| {
        Ok((row.get::<usize, u64>(0)?, row.get::<usize, u64>(1)?))
    }) {
        let elapsed = elapsed.min(previous_elapsed);
        let count = count + 1;

        conn.execute(
            "update timings set elapsed=?4, count=?5
                  where year=?1 and day=?2 and sha256=?3",
            (&year, &day, &digest, &elapsed, &count),
        )?;
    } else {
        conn.execute(
            "insert into timings (year,day,sha256,elapsed,count) values (?1, ?2, ?3, ?4, 1)",
            (&year, &day, &digest, &elapsed),
        )?;
    }

    Ok(())
}

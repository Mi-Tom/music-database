use rusqlite::{Connection, Result};

use crate::models::Song;

pub struct Database {
    conn: Connection,
}

impl Database {
    fn init(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS songs (
                id      TEXT PRIMARY KEY,
                title   TEXT NOT NULL,
                artists TEXT NOT NULL,
                album   TEXT,
                duration INTEGER,
                year    INTEGER
            );
        ")?;
        Ok(())
    }
    
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init()?;
        Ok(db)
    }

    pub fn write(&self, song: &Song) -> Result<()> {
        self.conn.execute(
            "INSERT INTO songs (id, title, artists, album, duration, year)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                song.uuid.to_string(),
                &song.title,
                &song.artists,
                &song.album,
                song.duration,
                song.year,
            ),
        )?;
        Ok(())
    }
}
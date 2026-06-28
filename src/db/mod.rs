use rusqlite::{Connection, Result};
use uuid::Uuid;

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

    pub fn get_songs(&self, limit: usize, offset: usize) -> Result<Vec<Song>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, artists, album, duration, year FROM songs LIMIT ?1 OFFSET ?2"
        )?;
    
        let songs = stmt.query_map([limit, offset], |row| {
            Ok(Song {
                uuid: row.get::<_, String>(0)?.parse().unwrap(),
                title: row.get(1)?,
                artists: row.get(2)?,
                album: row.get(3)?,
                duration: row.get(4)?,
                year: row.get(5)?,
                deleted: false
            })
        })?
        .collect::<Result<Vec<_>>>()?;
    
        Ok(songs)
    }

    pub fn get_song_by_id(&self, uuid: Uuid) -> Result<Option<Song>> {
        let uuid_string = uuid.to_string();
        let id = &uuid_string;
        let mut stmt = self.conn.prepare(
            "SELECT id, title, artists, album, duration, year FROM songs WHERE id = ?1"
        )?;
    
        let mut rows = stmt.query([id])?;
    
        match rows.next()? {
            Some(row) => Ok(Some(Song {
                uuid: row.get::<_, String>(0)?.parse().unwrap(),
                title: row.get(1)?,
                artists: row.get(2)?,
                album: row.get(3)?,
                duration: row.get(4)?,
                year: row.get(5)?,
                deleted: false
            })),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use crate::models::Song;

    #[test]
    fn database_init_open() {
        let file = NamedTempFile::new().unwrap();
        let result = Database::open(file.path().to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn database_write() {
        let file = NamedTempFile::new().unwrap();
        let db = Database::open(file.path().to_str().unwrap()).unwrap();
        assert!(db.write(&Song::add("Happy now", "Zedd, Elley Duhé")).is_ok())
    }

    #[test]
    fn database_get_songs() {
        let file = NamedTempFile::new().unwrap();
        let db = Database::open(file.path().to_str().unwrap()).unwrap();
        let mut original_array: Vec<Song> = Vec::new();
        for i in 0..=10 {
            let title = format!("title{}", i);
            let artist = format!("artist{}", i);
            original_array.push(Song::add(&title, &artist));
            db.write(original_array.get(i).unwrap()).unwrap();
        }
        let new_array = db.get_songs(5, 2).unwrap();
        for i in 2..new_array.iter().len() + 2 as usize {
            let j = i - 2;
            assert_eq!(original_array.get(i).unwrap().title, new_array.get(j).unwrap().title);
        }
    }

    #[test]
    fn database_get_song_by_id() {
        let file = NamedTempFile::new().unwrap();
        let db = Database::open(file.path().to_str().unwrap()).unwrap();
        let mut original_array: Vec<Song> = Vec::new();
        for i in 0..=10 {
            let title = format!("title{}", i);
            let artist = format!("artist{}", i);
            original_array.push(Song::add(&title, &artist));
            db.write(original_array.get(i).unwrap()).unwrap();
        }
        let new = db.get_song_by_id(original_array.get(4).unwrap().uuid).unwrap().unwrap();
        assert_eq!(new.title, original_array.get(4).unwrap().title);
    }
}
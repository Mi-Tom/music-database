use std::fmt::format;

use uuid::Uuid;

#[derive(Debug)]
pub struct Song {
    #[allow(dead_code)]
    uuid: Uuid,
    pub title: String,
    pub artists: String,
    pub album: Option<String>,
    pub duration: Option<i32>,
    pub year: Option<i32>,
    #[allow(dead_code)]
    deleted: bool
}
impl Song {
    pub fn add(title: &str, artist: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title: title.to_string(),
            artists: artist.to_string(),
            album: None,
            duration: None,
            year: None,
            deleted: false
        }
    }

    pub fn format_row(&self) -> String {
        let option_output: (String, String, String, String, String) = (
            if self.title.len() > 20 as usize {
                format!("{:<.17}...", self.title)
            } else {
                self.title.to_string()
            },
            if self.artists.len() > 20 as usize {
                format!("{:<.17}...", self.title)
            } else {
                self.artists.to_string()
            },
            match &self.album {
                Some(name) => name.to_string(),
                None => "____________________".to_string()
            },
            match &self.duration {
                Some(time) => time.to_string(),
                None => "________".to_string()
            },
            match &self.year {
                Some(year) => year.to_string(),
                None => "____".to_string()
            },
        );

        format!("{:<20} {:<20} {:<20} {:^8} {:4}", option_output.0, option_output.1, option_output.2, option_output.3, option_output.4)
    }
}
/*
    pub fn output(&self) {
        println!("Title                Artist/s             Album                duration year");
        println!("-------------------- -------------------- -------------------- -------- ----");
        for i in 0..self.len() {
            self.get(i).unwrap().output();
        }
        println!("----------------------------------------------------------------------------");
    }
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn song_add() {
        let song = Song::add("Imagine", "John Lennon");
        assert_eq!(song.title, "Imagine");
        assert_eq!(song.artists, "John Lennon");
    }

    #[test]
    fn song_format_row() {
        let mut song = Song::add("Faded", "Alan Walker");
        assert_eq!(song.format_row(), "Faded                Alan Walker          ____________________ ________ ____");
        song = Song::add("Like a Rolling Stones", "Bob Dylan");
        assert_eq!(song.format_row(), "Like a Rolling St... Bob Dylan            ____________________ ________ ____");
    }
}

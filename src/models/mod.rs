use uuid::Uuid;

#[derive(Debug)]
pub struct Song {
    pub uuid: Uuid,
    pub title: String,
    pub artists: String,
    pub album: Option<String>,
    pub duration: Option<i32>,
    pub year: Option<i32>,
    pub deleted: bool
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

    pub fn output(&self) {
        let option_output: (String, String, String) = (
            match &self.album {
                Some(name) => name.to_string(),
                None => "_________________".to_string()
            },
            match &self.duration {
                Some(time) => time.to_string(),
                None => "_____".to_string()
            },
            match &self.year {
                Some(year) => year.to_string(),
                None => "____".to_string()
            },
        );

        println!("{:.<20.17} {:.<20.17} {:.<20.17} {:^8} {:4}", &self.title, &self.artists, option_output.0, option_output.1, option_output.2);
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
}

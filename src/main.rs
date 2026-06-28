use music_database::models::Song;

fn main() {
    let song = Song::add("Faded", "Alan Walker");
    song.output();
}

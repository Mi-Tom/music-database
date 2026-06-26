use uuid::Uuid;

struct Song {
    uuid: Uuid,
    title: String,
    artists: String,
    album: Option<String>,
    duration: Option<i32>,
    year: Option<i32>
}

fn main() {
    println!("hi sky");
}

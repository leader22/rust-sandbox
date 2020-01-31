use async_std::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct Song {
  album: String,
  artist: String,
  name: String,
  duration: String,
  disc: String,
  track: String,
  path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Album {
  name: String,
  year: String,
  songs: Vec<Song>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Artist {
  name: String,
  albums: Vec<Album>,
}

fn main() -> Result<(), surf::Exception> {
    task::block_on(async {
        let uri = "https://music.lealog.net/dist/music.json";
        let res: Vec<Artist> = surf::get(uri).recv_json().await?;

        println!("Fetched {} items", res.len());

        let name = "KIRINJI";
        println!("Search for {}", name);

        let found = res.into_iter().find(|artist| artist.name == name);
        match found {
            Some(artist) => {
                println!("Found {} albums", artist.albums.len());
                artist.albums.into_iter().for_each(|album| println!("- {}", album.name))
            },
            None => println!("Not found...")
        }

        Ok::<(), surf::Exception>(())
    })
}

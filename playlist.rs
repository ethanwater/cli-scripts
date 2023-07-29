use rand::prelude::*;
use std::path::Path;
use std::{fs, io};

fn playlist<R>(root: R) -> Result<Vec<String>, io::Error>
where
    R: AsRef<Path>,
{
    let mut playlist: Vec<String> = Vec::new();
    for file in fs::read_dir(root)? {
        let _entry = file?.path();
        let entry = _entry.as_path().to_str().unwrap().to_string();
        if entry.ends_with(".mp3") {
            playlist.push(entry);
        }
    }
    Ok(playlist)
}

fn play_default(playlist: Vec<String>) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    for audio in playlist.iter() {
        let file = fs::File::open(audio).unwrap();
        sink.append(rodio::Decoder::new(io::BufReader::new(file)).unwrap());

        println!("now playing: {audio}");
        sink.sleep_until_end();
    }
}

fn play_shuffle(playlist: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    playlist.shuffle(&mut rng);

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    for audio in playlist.iter() {
        let file = fs::File::open(audio).unwrap();
        sink.append(rodio::Decoder::new(io::BufReader::new(file)).unwrap());

        println!("now playing: {audio}");
        sink.sleep_until_end();
    }
}

fn main() -> io::Result<()> {
    let root = Path::new("/Users/ethan/Music/LiquidDNB");
    let mut playlist = playlist(root)?;
    play_shuffle(&mut playlist);

    Ok(())
}

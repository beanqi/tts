use std::{fs::OpenOptions, io::Write};

use crate::tts::{edge::Edgetts, TTS};


#[test]
fn test_edge_tts() {
    let mut tts = Edgetts::default();
    tts.init().unwrap();
    let audio = tts.gen_audio("Japanization What the World Can Learn from Japans Lost Decades\n\n\n\n\n").unwrap();
    let mut file = OpenOptions::new().create(true).truncate(true).write(true).open("english.mp3").unwrap();
    file.write_all(&audio).unwrap();
    println!("audio len: {}", audio.len());
}
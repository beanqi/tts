use std::{fs::OpenOptions, io::Write};

use crate::tts::{edge::Edgetts, TTS};


#[test]
fn test_edge_tts() {
    let mut tts = Edgetts::default();
    tts.init().unwrap();
    let audio = tts.gen_audio("大家晚上好,欢迎进入直播间!").unwrap();
    let mut file = OpenOptions::new().create(true).truncate(true).write(true).open("english.mp3").unwrap();
    file.write_all(&audio).unwrap();
    println!("audio len: {}", audio.len());
}
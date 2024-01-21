use std::io::{BufRead, Write};

use epub_to_speech::tts::TTS;
use epub_to_speech::tts::edge::Edgetts;
use epub_to_speech::util::contains_chinese;

fn main() {
    let mut en_tts = Edgetts::new("en-US-AndrewNeural", "medium", "medium", "medium", "audio-24khz-48kbitrate-mono-mp3");
    let mut zh_tts = Edgetts::new("zh-CN-YunjianNeural", "medium", "medium", "medium", "audio-24khz-48kbitrate-mono-mp3");
    en_tts.init().unwrap();
    zh_tts.init().unwrap();
    let file = std::fs::File::open("日本化-Japanization What the World Can Learn from Japans Lost Decades (William Pesek).txt").unwrap();
    let mut mp3_file = std::fs::File::create("Chepter1.mp3").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    let mut count = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        print!("generate the lines: {}\n", count);
        let audio = gen_mp3(&mut en_tts, &mut zh_tts, &line);
        mp3_file.write_all(&audio).unwrap();
        count += 1;
        if count > 1013 {
            break;
        }
        line.clear();
    }
}

fn gen_mp3(en_tts: &mut Edgetts, zh_tts: &mut Edgetts, text: &str) -> Vec<u8> {
    let mut audio = Vec::new();
    loop {
        if text.is_empty() || text == "\n" {
            break;
        }

        if contains_chinese(text) {
            match zh_tts.gen_audio(text) {
                Ok(result) => {
                    audio = result;
                    break;
                }
                Err(_) => {
                    // Handle the error, e.g. retry or log the error
                    print!("zh_tts error: {}, try again\n", text);
                    continue;
                }
            }
        } else {
            match en_tts.gen_audio(text) {
                Ok(result) => {
                    audio = result;
                    break;
                }
                Err(_) => {
                    // Handle the error, e.g. retry or log the error
                    print!("en_tts error: {}, try again\n", text);
                    continue;
                }
            }
        }
    }
    audio
}
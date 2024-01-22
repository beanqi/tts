use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

use epub_to_speech::tts::TTS;
use epub_to_speech::tts::edge::Edgetts;
use epub_to_speech::util::contains_chinese;

fn main() {
    let mut en_tts = Edgetts::new("en-US-AndrewNeural", "medium", "medium", "loud", "audio-24khz-48kbitrate-mono-mp3");
    let mut zh_tts = Edgetts::new("zh-CN-YunjianNeural", "medium", "medium", "loud", "audio-24khz-48kbitrate-mono-mp3");
    en_tts.init().unwrap();
    zh_tts.init().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }
    let file = std::fs::File::open(&args[1]).unwrap();    

    let mut mp3_file = std::fs::File::create(format!("{}.mp3", args[1])).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        print!("generate the lines: {}\n", line);
        let audio = gen_mp3(&mut en_tts, &mut zh_tts, &line);
        mp3_file.write_all(&audio).unwrap();
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
                    // Handle the error, e.g. retry or log the error, sleep for a while
                    sleep(Duration::from_secs(60));
                    let _ = zh_tts.restart();
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
                    sleep(Duration::from_secs(60));
                    let _ = zh_tts.restart();
                    print!("en_tts error: {}, try again\n", text);
                    continue;
                }
            }
        }
    }
    audio
}
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

use epub_to_speech::tts::TTS;
use epub_to_speech::tts::edge::Edgetts;
use epub_to_speech::util::contains_chinese;
use indicatif::ProgressBar;

fn main() {
    let mut en_tts = Edgetts::new("en-US-AndrewNeural", "medium", "medium", "x-loud", "audio-24khz-96kbitrate-mono-mp3");
    let mut zh_tts = Edgetts::new("zh-CN-YunjianNeural", "medium", "medium", "x-loud", "audio-24khz-96kbitrate-mono-mp3");
    en_tts.init().unwrap();
    zh_tts.init().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    // Open file and read lines into a vector
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // the progress bar
    let total = lines.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // the checkpoint file
    let mut checkpoint_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}.checkpoint", args[1]))
        .unwrap();

    // Read checkpoint
    let mut checkpoint = String::new();
    checkpoint_file.read_to_string(&mut checkpoint).unwrap();
    let start_line: usize = checkpoint.trim().parse().unwrap_or(0);

    pb.set_position(start_line as u64);

    // Skip processed lines
    lines.drain(0..start_line);

    // the mp3 file
    let mut mp3_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}.mp3", args[1]))
        .unwrap();

    mp3_file.seek(SeekFrom::End(0)).unwrap();

    for line in lines {
        let audio = gen_mp3(&mut en_tts, &mut zh_tts, &line);
        mp3_file.write_all(&audio).unwrap();
        mp3_file.flush().unwrap();
        pb.inc(1);
        write_checkpoint(pb.position() as usize, &mut checkpoint_file);
    }
    pb.finish_with_message("done");
}

fn write_checkpoint(line: usize, checkpoint_file: &mut File) {
    checkpoint_file.seek(SeekFrom::Start(0)).unwrap();
    checkpoint_file.write_all(format!("{}", line).as_bytes()).unwrap();
    checkpoint_file.flush().unwrap();
}


fn gen_mp3(en_tts: &mut Edgetts, zh_tts: &mut Edgetts, text: &str) -> Vec<u8> {
    let audio;
    if contains_chinese(text) {
        audio = zh_tts.gen_audio(text).unwrap();
    } else {
        audio = en_tts.gen_audio(text).unwrap();
    }
    audio
}
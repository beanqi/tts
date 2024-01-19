

pub mod edge;

pub trait TTS {
    fn gen_audio(&mut self, text: &str) -> anyhow::Result<Vec<u8>>;
}
use std::net::TcpStream;
use anyhow::anyhow;

use rand::RngCore;
use tungstenite::{WebSocket, stream::MaybeTlsStream, Message, protocol::{CloseFrame, frame::coding::CloseCode}};
use xml::escape::{escape_str_attribute, escape_str_pcdata};

use super::TTS;



const SYNTH_URL: &str = "wss://speech.platform.bing.com/consumer/speech/synthesize/readaloud/edge/v1?TrustedClientToken=6A5AA1D4EAFF4E9FB37E23D68491D6F4";
/// 1. send config message to server through websocket
/// 2. send ssml message to server through websocket
/// 3. receive audio data from server through websocket
pub struct Edgetts {

    /// The voice to use for speech synthesis.
    /// like "zh-CN-XiaoxiaoNeural", not all voice are supported, you need to test by yourself, see https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/language-support#text-to-speech
    pub voice: String,

    /// The speech pitch.
    /// like x-low, low, medium, high, x-high, default
    pub pitch: String,

    /// The speech rate.
    /// like x-slow, slow, medium, fast, x-fast, default
    pub rate: String,

    /// The speech volume.
    /// like silent, x-soft, soft, medium, loud, x-loud, default
    pub volume: String,

    /// The output format
    /// like "audio-24khz-48kbitrate-mono-mp3", see https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/rest-text-to-speech#audio-outputs
    pub format: String,    

    pub ws: WebSocket<MaybeTlsStream<TcpStream>>
}

impl Default for Edgetts {
    fn default() -> Self {
        Self {
            voice: "zh-CN-XiaoxiaoNeural".to_string(),
            pitch: "medium".to_string(),
            rate: "medium".to_string(),
            volume: "x-loud".to_string(),
            format: "audio-24khz-96kbitrate-mono-mp3".to_string(),
            ws: tungstenite::connect(SYNTH_URL).unwrap().0,
        }
    }
}

impl Edgetts {
    pub fn new(voice: &str, pitch: &str, rate: &str, volume: &str, format: &str) -> Self {
        Self {
            voice: voice.to_string(),
            pitch: pitch.to_string(),
            rate: rate.to_string(),
            volume: volume.to_string(),
            format: format.to_string(),
            ws: tungstenite::connect(SYNTH_URL).unwrap().0,
        }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        self.ws.send(Message::Text(self.build_tts_config()))?;
        Ok(())
    }

    pub fn restart(&mut self) -> anyhow::Result<()> {
        self.close();
        self.ws = tungstenite::connect(SYNTH_URL).unwrap().0;
        self.ws.send(Message::Text(self.build_tts_config()))?;
        Ok(())
    }

    pub fn close(&mut self) {
        self.ws.close(Some(CloseFrame{
            code: CloseCode::Normal,
            reason: "".into(),
        })).unwrap();
    }


    /// build ssml
    fn build_ssml(&self, text: &str) -> String {
        format!(
            "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" xmlns:mstts=\"https://www.w3.org/2001/mstts\" xml:lang=\"en-US\">
                <voice name=\"{}\">
                    <prosody pitch=\"{}\" rate=\"{}\" volume=\"{}\">
                        {}
                    </prosody>
                </voice>
            </speak>", 
            escape_str_attribute(&self.voice), 
            escape_str_attribute(&self.pitch), 
            escape_str_attribute(&self.rate), 
            escape_str_attribute(&self.volume), 
            escape_str_pcdata(text)
        )
    }

    /// build tts config, only send to server once
    fn build_tts_config(&self) -> String {
        format!("Content-Type:application/json; charset=utf-8\r\nPath:speech.config\r\n\r\n{{\"context\":{{\"synthesis\":{{\"audio\":{{\"metadataoptions\":{{\"sentenceBoundaryEnabled\":false,\"wordBoundaryEnabled\":true}},\"outputFormat\":\"{}\"}}}}}}}}", self.format)
    }
    /// build tts request, send to server every time
    fn build_request(&self, text: &str) -> (String, String) {
        let request_id = self.random_request_id();
        (format!("X-RequestId:{}\r\nContent-Type:application/ssml+xml\r\nPath:ssml\r\n\r\n{}"
        , request_id
        , self.build_ssml(text)), request_id)
    }

    fn random_request_id(&self) -> String {
        let mut buf = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut buf);
        hex::encode(&buf[..])
    }

    fn parse_headers(&self, s: impl AsRef<str>) -> Vec<(String, String)> {
        s.as_ref().split("\r\n").filter_map(|s| {
            if s.len() > 0 {
                let mut iter = s.splitn(2, ":");
                let k = iter.next().unwrap_or("").to_owned();
                let v = iter.next().unwrap_or("").to_owned();
                Some((k, v))
            } else {
                None
            }
        }).collect()
    }
}


impl TTS for Edgetts {
    fn gen_audio(&mut self, text: &str) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let (request_body, request_id) = self.build_request(text);
        self.ws.send(tungstenite::Message::Text(request_body))?;
        loop {
            match self.ws.read() {
                Ok(msg) => {
                    match msg {
                        Message::Text(s) => {
                            if let Some(header_str) = s.splitn(2, "\r\n\r\n").next() {
                                let headers = self.parse_headers(header_str);
                                if headers.iter().any(|(k, v)| k == "Path" && v == "turn.end") {
                                    if headers.iter().any(|(k, v)| k == "X-RequestId" && v.as_str() == request_id) {
                                        return Ok(buf);
                                    } else {
                                        return Err(anyhow!("Path:turn.end no X-RequestId header"));
                                    }
                                }
                            } else {
                                return Err(anyhow!("bad text response. message not complete"));
                            }
                        }
                        Message::Binary(s) => {
                            let header_len = s[0] as usize * 256 + s[1] as usize;
                            if s.len() >= header_len + 2 {
                                let headers = self.parse_headers(String::from_utf8_lossy(&s[2..header_len]));
                                let body = &s[(header_len + 2)..];
                                if headers.iter().any(|(k, v)| k == "Path" && v == "audio") {
                                    if headers.iter().any(|(k, v)| k == "X-RequestId" && v.as_str() == request_id) {
                                        buf.extend(body);
                                    } else {
                                        return Err(anyhow!("Path:audio no X-RequestId header"));
                                    }
                                }
                            } else {
                                return Err(anyhow!("bad binary response. response len: {} header len: {}", s.len(), header_len));
                            }
                        }
                        _ => {}
                    };
                }
                Err(e) => {
                    return Err(anyhow!("socket read error: {:?}", e));
                }
            };
        }
    }
}

impl Drop for Edgetts {
    fn drop(&mut self) {
        self.ws.close(Some(CloseFrame{
            code: CloseCode::Normal,
            reason: "".into(),
        })).unwrap();
    }
}
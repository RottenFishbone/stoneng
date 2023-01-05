use rodio::{Source, Sink, OutputStream, Decoder};
use std::{
    sync::{
        Arc,
        mpsc::{self, Sender, Receiver},
    },
    fs::File,
    io::{BufReader, Read, Cursor}, path::PathBuf,
};


/// A heap-allocated container for the audio file buffer
/// This follows the solution presented by Xaeoxe and sinesc here:
///     https://github.com/RustAudio/rodio/issues/141
#[derive(Debug, Clone)]
struct SoundData(Arc<Vec<u8>>);
impl AsRef<[u8]> for SoundData {
    fn as_ref(&self) -> &[u8] { &self.0 }
}
impl From<Vec<u8>> for SoundData {
    fn from(data: Vec<u8>) -> Self { Self(Arc::new(data)) }
}

#[derive(Debug, Clone, Copy)]
pub enum AudioType {
    Sfx,
    Music,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum AudioRequest {
    MusicVolume(f32),
    MusicPlay(AudioHandle),
    MusicQueue(AudioHandle),
    MusicResume,
    MusicPause,
    MusicStop,
    
    SfxVolume(f32),
    SfxQueue(AudioHandle),
    SfxResume,
    SfxPause,
    SfxStop,
}

/// AudioHandles serve as a container for loading and passing audio files.
///
/// All AudioHandles hold references to heap allocations of data and can be freely
/// duplicated as needed.
#[derive(Debug, Clone)]
pub struct AudioHandle {
    data: SoundData,
}
impl AudioHandle {
    /// Loads a new AudioHandle into memory.
    /// 
    /// This causes an immediate read.
    /// Note, for larger files it may be prudent to drop this after queuing for
    /// playback.
    /// 
    /// Panics on IO error.
    pub fn load(path: PathBuf) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        let _ = file.read_to_end(&mut data);

        Self { data: SoundData::from(data) }
    }
    
    /// Builds a new Decoder from AudioHandle to be used for playback
    ///
    /// This is consumed on playback and must be built each time. However, 
    /// the underlying file data is not duplicated/reloaded, it is merely referenced.
    fn decoder(&self) -> Decoder<Cursor<SoundData>> {
        Decoder::new(Cursor::new(self.data.clone())).unwrap()
    }
}

/// The AudioEngine serves as a manager/server for audio queueing and playback.
///
/// Audio playback is handled on a separate thread 
pub struct AudioEngine {
    audio_tx:   Sender<AudioRequest>,

    music_volume:   f32,
    sfx_volume:     f32,
}
impl Default for AudioEngine {
    fn default() -> Self { Self::launch(3) }
}
impl AudioEngine {
    // TODO switch to manually managing queues to modify playback speed per track
    pub fn launch(max_sfx: usize) -> Self {
        let (tx, rx) = mpsc::channel::<AudioRequest>();

        std::thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            
            let mut sfx_sinks = Vec::new();
            for _ in 0..max_sfx {
                sfx_sinks.push(Sink::try_new(&stream_handle).unwrap());
            }
            let bgm_sink = Sink::try_new(&stream_handle).unwrap();
    
            loop {
                match rx.recv() {
                    Ok(request) => {
                        match request {
                            // SFX Playback
                            AudioRequest::SfxStop => for sink in &sfx_sinks { sink.stop() },
                            AudioRequest::SfxPause => for sink in &sfx_sinks { sink.pause() },
                            AudioRequest::SfxResume => for sink in &sfx_sinks { sink.play() },
                            AudioRequest::SfxQueue(audio) => {
                                // Find the emptiest sink
                                let sink = sfx_sinks.iter()
                                    .min_by(|a,b| a.len().cmp(&b.len()))
                                    .unwrap();
                                sink.append(audio.decoder());
                            },
                            
                            // Music Playback
                            AudioRequest::MusicStop => bgm_sink.stop(),
                            AudioRequest::MusicPause => bgm_sink.pause(),
                            AudioRequest::MusicResume => bgm_sink.play(),
                            AudioRequest::MusicQueue(audio) => bgm_sink.append(audio.decoder()),
                            AudioRequest::MusicPlay(audio) => {
                                bgm_sink.stop();
                                bgm_sink.append(audio.decoder());
                            },
                            
                            // Volume Control
                            AudioRequest::MusicVolume(level) => bgm_sink.set_volume(level),
                            AudioRequest::SfxVolume(level) =>
                                for sink in &sfx_sinks { sink.set_volume(level); },
                        }
                    },
                    // Exit the thread once the sender is disconnected (Engine dropped)
                    Err(err) => break,
                }
            }
        });

        Self { audio_tx: tx, music_volume: 1.0, sfx_volume: 1.0 }
    }

    pub fn get_volumes(&self) -> (f32, f32) { return (self.music_volume, self.sfx_volume); }
    pub fn set_volume(&mut self, audio_type: AudioType, level: f32) {
        // Build the appropriate request
        let request = match audio_type {
            AudioType::Sfx => {
                self.sfx_volume = level;
                AudioRequest::SfxVolume(level)
            },
            AudioType::Music => {
                self.music_volume = level;
                AudioRequest::MusicVolume(level)
            },
        };
        
        // Send to manager thread
        let _ = self.audio_tx.send(request);
    }
    pub fn play_music(&self) {
        //TODO implement
        let ah = AudioHandle::load(PathBuf::from("assets/audio/bgm/8BitMenu_loop.mp3")); 

        let request = AudioRequest::MusicVolume(0.25);
        let _ = self.audio_tx.send(request);
        let request = AudioRequest::MusicPlay(ah);
        let _ = self.audio_tx.send(request);
    }   

    pub fn play_sfx(&self) {
        //TODO implement
        let ah = AudioHandle::load(PathBuf::from("assets/audio/sfx/impact4.wav")); 

        let request = AudioRequest::SfxVolume(0.25);
        let _ = self.audio_tx.send(request);
        let request = AudioRequest::SfxQueue(ah);
        let _ = self.audio_tx.send(request);
    }
}

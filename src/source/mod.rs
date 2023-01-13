//! Asset loaders for commonly used audio file formats

#[cfg(feature = "flac")]
pub mod flac_loader;
#[cfg(feature = "mp3")]
pub mod mp3_loader;
#[cfg(feature = "ogg")]
pub mod ogg_loader;
#[cfg(feature = "settings_loader")]
pub mod settings_loader;
#[cfg(feature = "wav")]
pub mod wav_loader;

use bevy::reflect::TypeUuid;
use kira::sound::static_sound::StaticSoundData;
use kira::sound::streaming::StreamingSoundData;

/// A source of audio data
#[derive(Clone, TypeUuid)]
#[uuid = "6a9fc4ca-b5b5-94d6-613c-522e2d9fe86d"]
pub struct AudioSource {
    /// The Kira sound making up this `AudioSource`
    pub sound: StaticSoundData,
}

/// A source of audio stream data
#[derive(TypeUuid)]
#[uuid = "eacfe6e5-7f6f-457b-be21-444067ab3de8"]
pub struct AudioStreamingSource {
    /// The Kira sound making up this `AudioSource`
    pub sound: StreamingSoundData,
}

unsafe impl Send for AudioStreamingSource {}
unsafe impl Sync for AudioStreamingSource {}

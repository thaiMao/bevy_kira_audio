use anyhow::Result;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::utils::BoxedFuture;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};
use std::io::Cursor;

use crate::source::{AudioSource, AudioStreamingSource};

#[derive(Default)]
pub struct WavLoader;

impl AssetLoader for WavLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            let mut sound_bytes = vec![];
            for byte in bytes {
                sound_bytes.push(*byte);
            }
            let sound = StaticSoundData::from_cursor(
                Cursor::new(sound_bytes),
                StaticSoundSettings::default(),
            )?;
            load_context.set_default_asset(LoadedAsset::new(AudioSource { sound }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}

#[derive(Default)]
pub struct StreamingWavLoader;

impl AssetLoader for StreamingWavLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            let mut sound_bytes = vec![];
            for byte in bytes {
                sound_bytes.push(*byte);
            }
            let sound = StreamingSoundData::from_cursor(
                Cursor::new(sound_bytes),
                StreamingSoundSettings::default(),
            )?;
            load_context.set_default_asset(LoadedAsset::new(AudioStreamingSource { sound }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}

use bevy::prelude::Res;
use bevy_kira_audio::{Audio, AudioPlugin, InstanceHandle};

pub struct LoopAudioInstanceHandle {
    pub(crate) instance_handle: InstanceHandle,
}
pub fn stop_music(audio : Res<Audio>) {
    audio.stop();
}
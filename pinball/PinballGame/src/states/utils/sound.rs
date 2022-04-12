use bevy::prelude::*;
use bevy::audio::*;

pub struct LoopAudioInstanceHandle(pub Handle<AudioSink>);

pub fn stop_music(    audio_sinks: Res<Assets<AudioSink>>,
                      music_controller: Res<LoopAudioInstanceHandle>,) {
    if let Some(sink) = audio_sinks.get(&music_controller.0){
        sink.pause();
    }
}
pub fn set_vol(
    volume: Res<crate::Volume>,
    music_controller: Res<LoopAudioInstanceHandle>,
    audio_sinks: Res<Assets<AudioSink>>,
){
    if let Some(sink) = audio_sinks.get(&music_controller.0){
        sink.set_volume(volume.0 as f32); 
    }
}
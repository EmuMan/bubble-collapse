use bevy::{prelude::*, utils::HashMap};

use crate::components::audio::AudioLimitRemovalFlag;

#[derive(Resource, Debug, Default, Clone)]
pub struct AudioLimiter {
    max_counts: HashMap<Handle<AudioSource>, u32>,
    current_counts: HashMap<Handle<AudioSource>, u32>,
}

impl AudioLimiter {
    pub fn new() -> Self {
        Self {
            max_counts: HashMap::new(),
            current_counts: HashMap::new(),
        }
    }

    pub fn set_limit(&mut self, audio: Handle<AudioSource>, limit: u32) {
        self.max_counts.insert(audio, limit);
    }

    pub fn get_limit(&self, audio: Handle<AudioSource>) -> Option<u32> {
        self.max_counts.get(&audio).cloned()
    }

    pub fn decrease_count(&mut self, audio: Handle<AudioSource>) {
        if let Some(count) = self.current_counts.get_mut(&audio) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    pub fn play_if_allowed<'a>(
        &mut self,
        commands: &'a mut Commands,
        audio: Handle<AudioSource>,
        playback_settings: PlaybackSettings,
    ) -> Option<EntityCommands<'a>> {
        let limit = self.get_limit(audio.clone()).unwrap_or(u32::MAX);

        {
            let count = self.current_counts.entry(audio.clone()).or_insert(0);
            if *count >= limit {
                return None;
            }
            *count += 1;
        }

        Some(commands.spawn((
            AudioPlayer::new(audio),
            playback_settings,
            AudioLimitRemovalFlag,
        )))
    }
}

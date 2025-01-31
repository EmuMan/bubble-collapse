use bevy::prelude::*;

use crate::{components::audio::AudioLimitRemovalFlag, resources::audio::AudioLimiter};

pub fn decrease_limiter_on_removal(
    trigger: Trigger<OnRemove, AudioLimitRemovalFlag>,
    mut audio_limiter: ResMut<AudioLimiter>,
    query: Query<&AudioPlayer>,
) {
    let entity = trigger.entity();
    if let Ok(player) = query.get(entity) {
        audio_limiter.decrease_count(player.0.clone());
    } else {
        warn!("Entity {:?} does not have an AudioPlayer component", entity);
    }
}

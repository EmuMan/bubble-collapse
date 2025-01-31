use bevy::prelude::*;

use crate::resources::audio::AudioLimiter;
use crate::systems::audio::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioLimiter>()
            .add_observer(decrease_limiter_on_removal);
    }

}

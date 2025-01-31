use bevy::prelude::*;

use crate::resources::cache::*;
use crate::systems::cache::*;

pub struct CachePlugin;

impl Plugin for CachePlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<MeshCache>()
            .init_resource::<FontCache>()
            .init_resource::<AudioCache>()
            .add_systems(Startup, (
                init_mesh_cache,
                init_font_cache,
                init_audio_cache,
            ));
    }

}

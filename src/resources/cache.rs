use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone)]
pub struct MeshCache {
    pub circle_mesh: Handle<Mesh>,
    pub long_rectangle_mesh: Handle<Mesh>,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct FontCache {
    pub coolvetica_rg: Handle<Font>,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct AudioCache {
    pub bubble_pop: Handle<AudioSource>,
    pub bubble_beam: Handle<AudioSource>,
    pub bubble_explosion: Handle<AudioSource>,
    pub bubble_black_hole: Handle<AudioSource>,
}

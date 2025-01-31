use bevy::prelude::*;

use crate::resources::{audio::AudioLimiter, cache::*};

pub fn init_mesh_cache(
    mut mesh_cache: ResMut<MeshCache>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    mesh_cache.circle_mesh = meshes.add(Circle::new(1.0));
    mesh_cache.long_rectangle_mesh = meshes.add(Rectangle::new(1.0, 2_000.0));
}

pub fn init_font_cache(
    mut font_cache: ResMut<FontCache>,
    asset_server: Res<AssetServer>,
) {
    font_cache.coolvetica_rg = asset_server.load("fonts/Coolvetica Rg.otf");
}

pub fn init_audio_cache(
    mut audio_cache: ResMut<AudioCache>,
    mut audio_limiter: ResMut<AudioLimiter>,
    asset_server: Res<AssetServer>,
) {
    audio_cache.bubble_pop = asset_server.load("sfx/sarooptech_bubble_pop.ogg");
    audio_limiter.set_limit(audio_cache.bubble_pop.clone(), 5);
}

use bevy::prelude::*;
use crate::resources::stats::*;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameStats>();
    }

}

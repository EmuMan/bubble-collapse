use bevy::prelude::*;

use crate::resources::bubbles::*;
use crate::systems::bubbles::spawning::*;
use crate::systems::bubbles::movement::*;

pub struct BubblesPlugin;

impl Plugin for BubblesPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<BubbleSpawnTimer>()
            .add_systems(Startup, init_bubble_spawner)
            .add_systems(Update, spawn_bubbles)
            .add_systems(Update, move_bubbles);
    }

}

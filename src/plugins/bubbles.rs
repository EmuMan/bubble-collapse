use bevy::prelude::*;

use crate::resources::bubbles::*;
use crate::systems::bubbles::spawning::*;
use crate::systems::bubbles::combat::*;
use crate::systems::bubbles::shockwave::*;

pub struct BubblesPlugin;

impl Plugin for BubblesPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<BubbleSpawnTimer>()
            .add_event::<BubbleDestroyedEvent>()
            .add_systems(Startup, init_bubble_spawner)
            .add_systems(Update, spawn_bubbles)
            .add_systems(Update, bubble_clicked)
            .add_systems(Update, spawn_shockwaves)
            .add_systems(Update, expand_shockwaves)
            .add_systems(Update, bubble_hit_by_shockwave)
            .add_systems(Update, advance_bubble_collapse
                .after(bubble_clicked)
                .after(bubble_hit_by_shockwave));
    }

}

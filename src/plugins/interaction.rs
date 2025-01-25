use bevy::prelude::*;

pub struct InteractionPlugin;

use crate::resources::interaction::*;
use crate::systems::interaction::*;

impl Plugin for InteractionPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_event::<MouseClickEvent>()
            .add_systems(First, handle_mouse_click);
    }

}

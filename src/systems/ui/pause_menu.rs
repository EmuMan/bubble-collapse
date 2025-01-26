use bevy::prelude::*;

use crate::{components::ui::PauseMenuRoot, game_states::{GameState, PausedState}, util};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseMenuAction {
    Continue,
    ReturnToMainMenu,
}

pub fn draw_pause_menu(
    mut commands: Commands,
) {
    let container = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        PauseMenuRoot,
    )).id();

    let main_text = commands.spawn((
        Text::new("Paused"),
        TextFont {
            font_size: 70.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            ..default()
        },
    )).id();

    let buttons_container = commands.spawn(Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(20.0),
        ..default()
    }).id();

    let continue_button = util::spawn_button_with_text(
        &mut commands, "Continue".into(), Val::Px(200.0), Val::Px(50.0), 25.0);
    commands.entity(continue_button).insert(PauseMenuAction::Continue);

    let main_menu_button = util::spawn_button_with_text(
        &mut commands, "Main Menu".into(), Val::Px(200.0), Val::Px(50.0), 25.0);
    commands.entity(main_menu_button).insert(PauseMenuAction::ReturnToMainMenu);
    
    commands.entity(buttons_container).add_children(&[continue_button, main_menu_button]);

    commands.entity(container).add_children(&[main_text, buttons_container]);
}

pub fn cleanup_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
    mut interaction_query: Query<
        (
            &PauseMenuAction,
            &Interaction,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (action, interaction, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::srgb(0.3, 0.5, 0.8);
                perform_action(&mut next_game_state, &mut next_paused_state, *action);
            }
            Interaction::Hovered => {
                border_color.0 = Color::srgb(0.5, 0.7, 1.0);
            }
            Interaction::None => {
                border_color.0 = Color::WHITE;
            }
        }
    }
}

fn perform_action(
    next_game_state: &mut NextState<GameState>,
    next_paused_state: &mut NextState<PausedState>,
    action: PauseMenuAction,
) {
    match action {
        PauseMenuAction::Continue => {
            next_paused_state.set(PausedState::Unpaused);
        }
        PauseMenuAction::ReturnToMainMenu => {
            next_game_state.set(GameState::MainMenu);
        }
    }
}

pub fn pause_game_on_esc(
    mut paused_state: ResMut<NextState<PausedState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        paused_state.set(PausedState::Paused);
    }
}

pub fn unpause_game_on_esc(
    mut paused_state: ResMut<NextState<PausedState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        paused_state.set(PausedState::Unpaused);
    }
}

pub fn unpause_game(
    mut paused_state: ResMut<NextState<PausedState>>,
) {
    paused_state.set(PausedState::Unpaused);
}

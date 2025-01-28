use bevy::prelude::*;

use crate::{components::ui::MainMenuRoot, game_states::GameState, util};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainMenuAction {
    Play,
    Options,
    Quit,
}

pub fn draw_main_menu(
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
        MainMenuRoot,
    )).id();

    let main_text = commands.spawn((
        Text::new("Bubble Collapse"),
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

    let play_button = util::spawn_button_with_text(
        &mut commands, "Play".into(), Val::Px(200.0), Val::Px(50.0), 25.0);
    commands.entity(play_button).insert(MainMenuAction::Play);

    let quit_button = util::spawn_button_with_text(
        &mut commands, "Quit".into(), Val::Px(200.0), Val::Px(50.0), 25.0);
    commands.entity(quit_button).insert(MainMenuAction::Quit);
    
    commands.entity(buttons_container).add_children(&[play_button, quit_button]);

    commands.entity(container).add_children(&[main_text, buttons_container]);
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (
            &MainMenuAction,
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
                perform_action(&mut next_game_state, &mut app_exit, *action);
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
    exit: &mut EventWriter<AppExit>,
    action: MainMenuAction,
) {
    match action {
        MainMenuAction::Play => {
            next_game_state.set(GameState::InGame);
        }
        MainMenuAction::Quit => {
            exit.send(AppExit::Success);
        }
        _ => {}
    }
}

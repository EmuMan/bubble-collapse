use bevy::prelude::*;

use crate::{components::{bubbles::BubbleType, ui::UpgradesMenuRoot}, resources::bubbles::{BubbleChances, BubbleSpawnTimer}, util};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpgradesMenuAction {
    IncreaseRate,
    IncreaseChance(BubbleType),
}

pub fn draw_upgrades_menu(
    mut commands: Commands,
) {
    let container = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            ..default()
        },
        UpgradesMenuRoot,
    )).id();

    let buttons_container = commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            padding: UiRect {
                left: Val::Px(20.0),
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                bottom: Val::Px(20.0),
            },
            ..default()
        },
    )).id();


    let rate_button = util::spawn_button_with_text(
        &mut commands, "Increase Rate".into(), Val::Px(150.0), Val::Px(35.0), 12.0);
    commands.entity(rate_button).insert(UpgradesMenuAction::IncreaseRate);

    let beam_button = util::spawn_button_with_text(
        &mut commands, "Beam Up".into(), Val::Px(150.0), Val::Px(35.0), 12.0);
    commands.entity(beam_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::Beam));

    let scatter_shot_button = util::spawn_button_with_text(
        &mut commands, "Scatter Shot Up".into(), Val::Px(150.0), Val::Px(35.0), 12.0);
    commands.entity(scatter_shot_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::ScatterShot));

    let black_hole_button = util::spawn_button_with_text(
        &mut commands, "Black Hole Up".into(), Val::Px(150.0), Val::Px(35.0), 12.0);
    commands.entity(black_hole_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::BlackHole));

    let mega_button = util::spawn_button_with_text(
        &mut commands, "Mega Up".into(), Val::Px(150.0), Val::Px(35.0), 12.0);
    commands.entity(mega_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::Mega));
    
    commands.entity(buttons_container).add_children(&[
        rate_button, beam_button, scatter_shot_button, black_hole_button, mega_button
    ]);

    commands.entity(container).add_children(&[buttons_container]);
}

pub fn cleanup_upgrades_menu(
    mut commands: Commands,
    query: Query<Entity, With<UpgradesMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut spawn_timer: ResMut<BubbleSpawnTimer>,
    mut spawn_chances: ResMut<BubbleChances>,
    mut interaction_query: Query<
        (
            &UpgradesMenuAction,
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
                perform_action(&mut spawn_timer, &mut spawn_chances, *action);
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
    spawn_timer: &mut BubbleSpawnTimer,
    spawn_chances: &mut BubbleChances,
    action: UpgradesMenuAction,
) {
    match action {
        UpgradesMenuAction::IncreaseRate => {
            spawn_timer.action_timer.amount += 10;
        }
        UpgradesMenuAction::IncreaseChance(bubble_type) => {
            spawn_chances.add_chance(bubble_type, 1.0);
        }
    }
}
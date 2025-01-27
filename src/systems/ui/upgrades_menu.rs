use bevy::prelude::*;

use crate::{components::{bubbles::BubbleType, ui::{Activatable, UpgradesMenuRoot}}, resources::{bubbles::{BubbleChances, BubbleSpawnTimer}, stats::GameStats, ui::{UpgradeChangedEvent, UpgradesMenuInfo}}, util};

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
        &mut commands, "Increase Rate".into(), Val::Px(200.0), Val::Px(35.0), 12.0);
    commands.entity(rate_button).insert(UpgradesMenuAction::IncreaseRate);
    commands.entity(rate_button).insert(Activatable::new(true));

    let beam_button = util::spawn_button_with_text(
        &mut commands, "Beam Up".into(), Val::Px(200.0), Val::Px(35.0), 12.0);
    commands.entity(beam_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::Beam));
    commands.entity(beam_button).insert(Activatable::new(true));

    let scatter_shot_button = util::spawn_button_with_text(
        &mut commands, "Scatter Shot Up".into(), Val::Px(200.0), Val::Px(35.0), 12.0);
    commands.entity(scatter_shot_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::ScatterShot));
    commands.entity(scatter_shot_button).insert(Activatable::new(true));

    let black_hole_button = util::spawn_button_with_text(
        &mut commands, "Black Hole Up".into(), Val::Px(200.0), Val::Px(35.0), 12.0);
    commands.entity(black_hole_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::BlackHole));
    commands.entity(black_hole_button).insert(Activatable::new(true));

    let mega_button = util::spawn_button_with_text(
        &mut commands, "Mega Up".into(), Val::Px(200.0), Val::Px(35.0), 12.0);
    commands.entity(mega_button).insert(UpgradesMenuAction::IncreaseChance(BubbleType::Mega));
    commands.entity(mega_button).insert(Activatable::new(true));
    
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

pub fn button_interactions(
    mut spawn_timer: ResMut<BubbleSpawnTimer>,
    mut spawn_chances: ResMut<BubbleChances>,
    mut upgrades: ResMut<UpgradesMenuInfo>,
    mut stats: ResMut<GameStats>,
    mut unlock_events: EventWriter<UpgradeChangedEvent>,
    mut interaction_query: Query<
        (
            &UpgradesMenuAction,
            &Interaction,
            &mut BorderColor,
            &Activatable,
        ),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (action, interaction, mut border_color, activatable) in &mut interaction_query {
        if !activatable.active {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::srgb(0.3, 0.5, 0.8);
                let old_cost = *upgrades.costs.get(action).unwrap_or(&0);
                if stats.score < old_cost as i32 {
                    continue;
                }
                upgrades.costs.insert(*action, (old_cost as f32 * 1.1) as u32);
                stats.score -= old_cost as i32;
                perform_action(&mut spawn_timer, &mut spawn_chances, *action);
                unlock_events.send(UpgradeChangedEvent { action: *action });
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

pub fn update_upgrades_menu(
    upgrades: Res<UpgradesMenuInfo>,
    mut unlock_events: EventReader<UpgradeChangedEvent>,
    mut button_query: Query<(&UpgradesMenuAction, &Children, &mut Activatable, &mut BorderColor), With<Button>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    for event in unlock_events.read() {
        for (action, children, mut activatable, mut border_color) in &mut button_query {
            if event.action == *action {
                let (mut text, mut text_color) = text_query.get_mut(children[0]).unwrap();

                let string = text.0.clone();
                let text_without_score = string.split(" (").collect::<Vec<&str>>()[0];
                text.0 = format!("{} ({})", text_without_score, upgrades.costs.get(action).unwrap_or(&0));

                activatable.active = upgrades.is_unlocked(*action);

                if activatable.active {
                    border_color.0 = Color::WHITE;
                    text_color.0 = Color::WHITE;
                } else {
                    border_color.0 = Color::srgb(0.2, 0.4, 0.7);
                    text_color.0 = Color::srgb(0.0, 0.0, 1.0);
                }
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
            spawn_timer.action_timer.amount += 50 / spawn_timer.action_timer.amount + 1;
        }
        UpgradesMenuAction::IncreaseChance(bubble_type) => {
            spawn_chances.add_chance(bubble_type, 1.0);
        }
    }
}

pub fn reset_upgrades(
    mut commands: Commands,
    mut unlock_events: EventWriter<UpgradeChangedEvent>,
) {
    commands.init_resource::<UpgradesMenuInfo>();
    unlock_events.send(UpgradeChangedEvent { action: UpgradesMenuAction::IncreaseRate });
    unlock_events.send(UpgradeChangedEvent { action: UpgradesMenuAction::IncreaseChance(BubbleType::Beam) });
    unlock_events.send(UpgradeChangedEvent { action: UpgradesMenuAction::IncreaseChance(BubbleType::ScatterShot) });
    unlock_events.send(UpgradeChangedEvent { action: UpgradesMenuAction::IncreaseChance(BubbleType::BlackHole) });
    unlock_events.send(UpgradeChangedEvent { action: UpgradesMenuAction::IncreaseChance(BubbleType::Mega) });
}

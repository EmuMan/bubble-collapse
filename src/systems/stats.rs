use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::ui::ScoreText, resources::{bubbles::BubbleCollapsedEvent, stats::GameStats, ui::{UpgradeChangedEvent, UpgradesMenuInfo}}, util::get_viewport_bounds};

pub fn init_stats(mut game_stats: ResMut<GameStats>) {
    game_stats.score = 0;
}

pub fn draw_score(
    mut commands: Commands,
    game_stats: Res<GameStats>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Coolvetica Rg.otf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    commands.spawn((
        Text2d::new(format!("Score: {}", game_stats.score)),
        text_font,
        TextLayout::new_with_justify(JustifyText::Center),
        ScoreText::default(),
        Transform::from_translation(Vec3::new(-400.0, 300.0, 0.0)),
    ));
}

pub fn cleanup_score(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_score(
    mut commands: Commands,
    time: Res<Time>,
    game_stats: Res<GameStats>,
    mut score_query: Query<(Entity, &mut Transform, &mut ScoreText)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let viewport_bounds = get_viewport_bounds(&window_query, &camera_query);
    
    let position = viewport_bounds.map(|bounds| Vec3::new(bounds.min.x + 200.0, bounds.max.y - 100.0, 0.0));

    for (entity, mut transform, mut score_text) in &mut score_query {
        commands.entity(entity).insert(Text2d::new(format!("Score: {}", game_stats.score)));
        score_text.scale_timer.tick(time.delta());
        let left = score_text.scale_timer.remaining_secs() / score_text.scale_timer.duration().as_secs_f32();
        let scale = 1.0 + 0.2 * left;
        transform.scale = Vec3::new(scale, scale, 1.0);
        if let Some(position) = position {
            transform.translation = position;
        }
    }
}

pub fn increment_score_for_destroyed_bubbles(
    mut game_stats: ResMut<GameStats>,
    mut bubble_collapse_event: EventReader<BubbleCollapsedEvent>,
    mut score_text_query: Query<&mut ScoreText>,
) {
    for collapse in bubble_collapse_event.read() {
        if !collapse.triggered_by_user {
            game_stats.score += collapse.score_change;
            for mut score_text in &mut score_text_query {
                score_text.scale_timer.reset();
            }
        }
    }
}

pub fn unlock_upgrades(
    game_stats: Res<GameStats>,
    mut upgrades: ResMut<UpgradesMenuInfo>,
    mut unlock_events: EventWriter<UpgradeChangedEvent>,
) {
    let mut to_unlock = vec![];
    for (upgrade, cost) in &upgrades.costs {
        if game_stats.score > 0 && game_stats.score as u32 >= *cost {
            to_unlock.push(*upgrade);
        }
    }
    for upgrade in to_unlock {
        upgrades.unlock(upgrade);
        unlock_events.send(UpgradeChangedEvent { action: upgrade });
    }
}

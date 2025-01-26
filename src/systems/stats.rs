use bevy::prelude::*;

use crate::{components::ui::ScoreText, resources::{bubbles::BubbleCollapsedEvent, stats::GameStats}};

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
    time: Res<Time>,
    game_stats: Res<GameStats>,
    mut query: Query<(&mut Text2d, &mut TextFont, &mut ScoreText)>,
) {
    for (mut text, mut text_font, mut score_text) in &mut query {
        text.0 = format!("Score: {}", game_stats.score);
        score_text.scale_timer.tick(time.delta());
        let left = score_text.scale_timer.remaining_secs() / score_text.scale_timer.duration().as_secs_f32();
        text_font.font_size = 50.0 + 20.0 * left;
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

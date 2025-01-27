use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct MainMenuRoot;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PauseMenuRoot;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UpgradesMenuRoot;

#[derive(Component, Debug, Clone)]
pub struct ScoreText {
    pub scale_timer: Timer,
}

impl Default for ScoreText {
    fn default() -> Self {
        Self {
            scale_timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Activatable {
    pub active: bool,
}

impl Activatable {
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}

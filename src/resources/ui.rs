use bevy::{prelude::*, utils::HashMap};

use crate::{components::bubbles::BubbleType, systems::ui::upgrades_menu::UpgradesMenuAction};

#[derive(Resource, Debug)]
pub struct UpgradesMenuInfo {
    pub unlocked: Vec<UpgradesMenuAction>,
    pub costs: HashMap<UpgradesMenuAction, u32>,
}

impl Default for UpgradesMenuInfo {
    fn default() -> Self {
        Self {
            unlocked: vec![UpgradesMenuAction::IncreaseRate],
            costs: {
                let mut map = HashMap::default();
                map.insert(UpgradesMenuAction::IncreaseRate, 10);
                map.insert(UpgradesMenuAction::IncreaseChance(BubbleType::Beam), 25);
                map.insert(UpgradesMenuAction::IncreaseChance(BubbleType::ScatterShot), 150);
                map.insert(UpgradesMenuAction::IncreaseChance(BubbleType::BlackHole), 1000);
                map.insert(UpgradesMenuAction::IncreaseChance(BubbleType::Mega), 100000);
                map
            },
        }
    }
}

impl UpgradesMenuInfo {
    pub fn is_unlocked(&self, action: UpgradesMenuAction) -> bool {
        self.unlocked.contains(&action)
    }

    pub fn unlock(&mut self, action: UpgradesMenuAction) {
        self.unlocked.push(action);
    }
}

#[derive(Event, Debug)]
pub struct UpgradeChangedEvent {
    pub action: UpgradesMenuAction,
}

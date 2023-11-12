// TODO: level system w/ YOLECK
use bevy::prelude::*;
use bevy_yoleck::prelude::YoleckLoadingCommand;

use crate::{GameplaySet, GameState};

// SUBMODULES
pub mod platform;
pub mod resizable;
pub mod text;

// RESOURCES
#[derive(Resource)]
pub struct CurrentLevel(Option<&'static str>);

// SYSTEMS
/// Set up the current level
pub fn setup(asset_server: Res<AssetServer>, mut yoleck_loading_cmd: ResMut<YoleckLoadingCommand>, current_level: Res<CurrentLevel>) {
    // TODO: menu
    if let Some(lvl) = current_level.0 {
        *yoleck_loading_cmd = YoleckLoadingCommand::FromAsset(asset_server.load(lvl));
    }
}

// TODO: MENU
fn unset_level(mut lvl: ResMut<CurrentLevel>) {
    lvl.0 = None;
}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentLevel(Some("levels/level1.yol")))
            .add_plugins((platform::Plugin, resizable::Plugin, text::Plugin))
            .add_systems(OnExit(GameState::InGame), unset_level)
            .add_systems(OnEnter(GameState::LevelEditor), unset_level)
            .add_systems(OnEnter(GameState::InGame), setup);
    }
}

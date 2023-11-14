// TODO: level system w/ YOLECK
use bevy::prelude::*;
use bevy_yoleck::{prelude::*, YoleckManaged};

use crate::{GameplaySet, GameState};

// SUBMODULES
pub mod platform;
pub mod resizable;
pub mod text;
// TODO: pub mod scroll_stop;
pub mod level_portal;

// RESOURCES
#[derive(Resource)]
pub struct CurrentLevel(Option<usize>);

// SYSTEMS
pub fn load_level(
    mut cmd: Commands,
    level_entities_query: Query<Entity, With<YoleckManaged>>,
    mut level_index_handle: Local<Option<Handle<YoleckLevelIndex>>>,
    level_index_assets: Res<Assets<YoleckLevelIndex>>, 
    asset_server: Res<AssetServer>,
    mut yoleck_loading_cmd: ResMut<YoleckLoadingCommand>,
    current_level: Res<CurrentLevel>,
    mut state: ResMut<NextState<GameState>>,
) {
    if let Some(level) = current_level.0 {
        let level_index_handle: Handle<YoleckLevelIndex> = level_index_handle
            .get_or_insert_with(|| asset_server.load("levels/index.yoli"))
            .clone();
        let Some(level_index) = level_index_assets.get(&level_index_handle) else { return };
        bevy::log::info!("Loading level {level}");

        for entity in level_entities_query.iter() {
            cmd.entity(entity).despawn_recursive();
        }

        let level_handle: Handle<YoleckRawLevel> = asset_server.load(
            format!("levels/{}", level_index[level - 1].filename)
        );
        *yoleck_loading_cmd = YoleckLoadingCommand::FromAsset(level_handle);
        state.set(GameState::InGame);
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
            .insert_resource(CurrentLevel(Some(1)))
            .add_plugins((level_portal::Plugin, platform::Plugin, resizable::Plugin, text::Plugin))
            .add_systems(OnExit(GameState::InGame), unset_level.run_if(not(in_state(GameState::Loading))))
            .add_systems(OnEnter(GameState::LevelEditor), unset_level)
            .add_systems(Update, load_level.run_if(in_state(GameState::Loading)));
    }
}

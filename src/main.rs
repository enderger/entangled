#![allow(clippy::type_complexity)]

use bevy::{prelude::*, window::WindowResolution};
use bevy_xpbd_2d::prelude::*;
use argh::FromArgs;
use bevy_yoleck::{bevy_egui::EguiPlugin, YoleckPluginForEditor, YoleckPluginForGame, prelude::YoleckSyncWithEditorState};

// SUBMODULES
mod camera;
mod level;
mod player;

// GAME STATES
#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum GameState {
    // TODO: #[default] Menu,
    #[default]
    InGame,
    LevelEditor,
}

// SYSTEM SETS
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameplaySet {
    Input,
    Update,
    Movement,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EditorSet;

// CLI
#[derive(FromArgs)]
/// A game about resizing platforms to solve puzzles
struct Entangled {
    #[argh(switch, short='d')]
    /// enable debugging
    debug: bool,

    #[argh(switch, short='e')]
    /// use the level editor
    editor: bool,
}

// MAIN
fn main() {
    let args: Entangled = argh::from_env();

    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Entangled"),
                    resolution: WindowResolution::from(camera::WINDOW_SIZE),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),));

    if !args.editor {
        app
            .add_plugins((YoleckPluginForGame,));
    } else {
        app
            .add_plugins((
                YoleckPluginForEditor,
                YoleckSyncWithEditorState {
                    when_editor: GameState::LevelEditor,
                    when_game: GameState::InGame,
                },
                EguiPlugin,
            ));
    }

    app
        .add_plugins((camera::Plugin, level::Plugin, player::Plugin, PhysicsPlugins::default(),))
        .add_state::<GameState>()
        .configure_sets(Update, (GameplaySet::Input, GameplaySet::Update, GameplaySet::Movement).chain().run_if(in_state(GameState::InGame)))
        .configure_sets(Update, EditorSet.run_if(in_state(GameState::LevelEditor)))
        .insert_resource(Gravity(Vec2::NEG_Y * 200.));

    if args.debug {
        // TODO: Debug plugin w/ Egui
        app.add_plugins((bevy_xpbd_2d::plugins::debug::PhysicsDebugPlugin::default(),));
    }

    app.run()
}


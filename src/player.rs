use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::prelude::*;

use crate::{GameState, GameplaySet, camera::WINDOW_BOTTOM_LEFT};

use self::movement::MovementBundle;

// SUBMODULES
pub mod movement;
pub mod respawn;

// BUNDLE
#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: SpriteBundle,
    movement: MovementBundle,
    marker: Player,
}

// SYSTEMS
/// The system which adds the player to the game
pub fn setup(
    mut cmd: Commands,
    assets: Res<AssetServer>,
) {
    cmd.spawn((PlayerBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25., 45.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(WINDOW_BOTTOM_LEFT.x + 100., -100., 10.),
            texture: assets.load("sprites/player.png"),
            ..Default::default()
        },
        movement: MovementBundle::new(Collider::capsule(20.0, 12.5)),
        marker: Player,
    },));
}

pub fn stop(
    mut cmd: Commands,
    mut q: Query<Entity, With<Player>>
) {
    q.for_each_mut(|player| cmd.entity(player).despawn());
}

// PLUGIN
/// The plugin that adds all data needed for players
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), (setup,))
            .add_systems(OnExit(GameState::InGame), (stop,))
            .add_plugins((movement::Plugin, respawn::Plugin));
    }
}


use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::prelude::*;

use crate::GameState;

use self::movement::MovementBundle;

// SUBMODULES
pub mod movement;

// BUNDLE
#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    movement: MovementBundle,
    marker: Player,
}

// SYSTEMS
/// The system which adds the player to the game
pub fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn((PlayerBundle {
        mesh: MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Capsule {
                        radius: 12.5,
                        depth: 20.0,
                        ..default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.2, 0.7, 0.9))),
            transform: Transform::from_xyz(crate::WINDOW_BOTTOM_LEFT.x + 150., -100.0, 0.0),
            ..default()
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
            .add_plugins(movement::Plugin);
    }
}


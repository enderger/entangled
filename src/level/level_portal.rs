use bevy::prelude::*;
use bevy_xpbd_2d::{prelude::*, math::*};
use bevy_yoleck::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{player::Player, GameplaySet, GameState};

use super::CurrentLevel;

// COMPONENTS
#[derive(Component)]
pub struct LevelTransition(usize);

// BUNDLE
#[derive(Bundle)]
pub struct LevelPortalBundle {
    sprite: SpriteBundle,
    trigger: ShapeCaster,
    transition: LevelTransition,
}

impl LevelPortalBundle {
    pub fn new(position: Vec2, level: usize, asset_server: &AssetServer) -> Self {
        let texture: Handle<Image> = asset_server.load("sprites/level_portal.png");

        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(25., 40.)),
                    ..Default::default()
                },
                transform: Transform::from_translation(position.extend(1.)),
                texture,
                ..Default::default()
            },
            trigger: ShapeCaster::new(Collider::capsule(12.5, 20.), Vector::new(position.x, position.y), 0.0, Vector::ONE)
                .with_max_time_of_impact(10.)
                .with_max_hits(8),
            transition: LevelTransition(level)
        }
    }
}

// SYSTEMS
pub fn handle_portal_interactions(
    transition_query: Query<(&ShapeHits, &LevelTransition), With<LevelTransition>>,
    player_query: Query<Has<Player>>,
    mut state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (hits, transition) in transition_query.iter() {
        if hits.iter().any(|data| player_query.get(data.entity).unwrap_or(false)) {
            current_level.0 = Some(transition.0);
            state.set(GameState::Loading);
        }
    }
}

// YOLECK
#[derive(Component, YoleckComponent, Serialize, Deserialize, Clone, PartialEq)]
pub struct YoleckPortal {
    pos: Vec2,
    target: usize,
}

impl Default for YoleckPortal {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            target: 1,
        }
    }
}

fn populate_portal(mut pop: YoleckPopulate<&YoleckPortal>, asset_server: Res<AssetServer>) {
    pop.populate(|_ctx, mut cmd, portal| {
        cmd.insert(LevelPortalBundle::new(
            portal.pos,
            portal.target,
            &asset_server
        ));
    })
}


fn edit_portal(mut ui: ResMut<YoleckUi>, mut edit: YoleckEdit<&mut YoleckPortal>) {
    if let Ok(mut portal) = edit.get_single_mut() {
        ui.add(egui::DragValue::new(&mut portal.target).prefix("Level: "));

        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut portal.pos.x).speed(1.).fixed_decimals(0).prefix("X: "));
            ui.add(egui::DragValue::new(&mut portal.pos.y).speed(1.).fixed_decimals(0).prefix("Y: "));
        });
    }
}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_portal_interactions.in_set(GameplaySet::Update));
        app.add_yoleck_entity_type(YoleckEntityType::new("Portal")
            .with::<YoleckPortal>()
        );
        app.add_yoleck_edit_system(edit_portal);
        app.yoleck_populate_schedule_mut().add_systems(populate_portal);
    }
}

// Systems

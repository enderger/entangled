use bevy::prelude::*;
use bevy_xpbd_2d::{prelude::*, math::Scalar};
use bevy_yoleck::prelude::*;
use serde::{Deserialize, Serialize};

// BUNDLE
#[derive(Bundle)]
pub struct PlatformBundle {
    sprite: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { 
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos.extend(0.1)),
                ..Default::default()
            },
            body: RigidBody::Static,
            collider: Collider::cuboid(size.x, size.y),
        }
    }
}

// LEVEL EDITOR
#[derive(Clone, PartialEq, Serialize, Deserialize, Component, YoleckComponent)]
pub struct YoleckPlatform {
    width: Scalar,
    height: Scalar,
    x: Scalar,
    y: Scalar,
}

impl Default for YoleckPlatform {
    fn default() -> Self {
        Self {
            width: 50.,
            height: 50.,
            x: 0.,
            y: 0.,
        }
    }
}

fn populate_platform(mut pop: YoleckPopulate<&YoleckPlatform>) {
    pop.populate(|_ctx, mut cmd, platform| {
        cmd.insert(PlatformBundle::new(
            Vec2::new(platform.x, platform.y),
            Vec2::new(platform.width, platform.height),
        )); 
    })
}

fn edit_platform(mut ui: ResMut<YoleckUi>, mut edit: YoleckEdit<&mut YoleckPlatform>) {
    if let Ok(mut platform) = edit.get_single_mut() {
        ui.add(egui::Slider::new(&mut platform.width, 50.0..=2000.0).prefix("Width: "));
        ui.add(egui::Slider::new(&mut platform.height, 50.0..=1000.0).prefix("Height: "));

        // TODO: Leave room for scale factor UI
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.add(egui::DragValue::new(&mut platform.x).speed(1.).fixed_decimals(0).prefix("X: "));

            // TODO: clamp range to height of scale factor UI - INFINITY
            ui.add(egui::DragValue::new(&mut platform.y).speed(1.).fixed_decimals(0).prefix("Y: "));
        });
    }
}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type(YoleckEntityType::new("Platform").with::<YoleckPlatform>());
        app.add_yoleck_edit_system(edit_platform);
        app.yoleck_populate_schedule_mut().add_systems(populate_platform);
    }
}

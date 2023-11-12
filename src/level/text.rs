use bevy::{text::{Text, Text2dBundle, TextStyle}, prelude::*};
use bevy_xpbd_2d::math::Scalar;
use bevy_yoleck::prelude::*;
use serde::{Serialize, Deserialize};

// YOLECK
#[derive(Clone, PartialEq, Serialize, Deserialize, Component, YoleckComponent)]
pub struct YoleckText {
    text: String,
    x: Scalar,
    y: Scalar,
    size: f32,
}

impl Default for YoleckText {
    fn default() -> Self {
        Self {
            text: String::from("TEXT"),
            x: 0.,
            y: 0.,
            size: 32.,
        }
    }
}

fn populate_text(mut pop: YoleckPopulate<&YoleckText>) {
    pop.populate(|_ctx, mut cmd, text| {
        cmd.insert(Text2dBundle {
            text: Text::from_section(text.text.clone(), TextStyle {
                font_size: text.size,
                color: Color::ANTIQUE_WHITE,
                ..Default::default()
            }),
            transform: Transform::from_xyz(text.x, text.y, 1.),
            ..Default::default()
        });
    });
}

fn edit_text(mut ui: ResMut<YoleckUi>, mut edit: YoleckEdit<&mut YoleckText>) {
    if let Ok(mut text) = edit.get_single_mut() {
        ui.add(egui::Slider::new(&mut text.size, 8.0..=128.).prefix("Size: "));

        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut text.x).speed(1.).fixed_decimals(0).prefix("X: "));

            ui.add(egui::DragValue::new(&mut text.y).speed(1.).fixed_decimals(0).prefix("Y: "));
        });

        ui.label("Text");
        ui.add(egui::text_edit::TextEdit::multiline(&mut text.text));
    }
}

// PLUGIN

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type(YoleckEntityType::new("Text").with::<YoleckText>());
        app.add_yoleck_edit_system(edit_text);
        app.yoleck_populate_schedule_mut().add_systems(populate_text);
    }
}

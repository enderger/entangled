use bevy::{prelude::*, utils::HashMap, input::mouse::{MouseWheel, MouseScrollUnit}};
use bevy_xpbd_2d::{prelude::*, math::Scalar};
use bevy_yoleck::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::{GameplaySet, EditorSet, GameState};


// COMPONENTS
#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, EnumIter, Serialize, Deserialize, Debug, PartialOrd, Ord)]
#[repr(u8)]
pub enum ScaleGroup {
    Red, Green, Blue,
}

#[derive(Resource)]
pub struct SelectedGroup(ScaleGroup);

impl From<ScaleGroup> for Color {
    fn from(group: ScaleGroup) -> Self {
        match group {
            ScaleGroup::Red => Color::RED,
            ScaleGroup::Green => Color::DARK_GREEN,
            ScaleGroup::Blue => Color::BLUE,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, EnumIter)]
#[repr(u8)]
pub enum ScaleDirection {
    Up, Down, Left, Right
}

#[derive(Component)]
pub struct Scalable {
    factor: Scalar,
    direction: ScaleDirection,
    position: Vec2,
    size: Vec2,
    bounds: std::ops::RangeInclusive<Scalar>,
}

#[derive(Component)]
pub struct Scale(Scalar);

// EVENTS
#[derive(Event)]
pub struct SelectFactorEvent {
    direction: i8,
}

#[derive(Event)]
pub struct ChangeScaleEvent(Scalar);

// BUNDLES
#[derive(Bundle)]
pub struct ScalableBundle {
    sprite: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    scalable: Scalable,
    group: ScaleGroup,
}

impl ScalableBundle {
    pub fn new(scalable: Scalable, size: Vec2, group: ScaleGroup) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: group.into(),
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform {
                    translation: scalable.position.extend(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Static,
            collider: Collider::cuboid(size.x, size.y),
            scalable, group,
        }
    }
}

// SYSTEMS
fn handle_keyboard_input(keyboard_input: Res<Input<KeyCode>>, mut select_factor_evw: EventWriter<SelectFactorEvent>) {
    let mut direction = 0i8;

    if keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        direction = 1; 
    }

    if keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        direction = -1; 
    }

    if direction != 0 {
        select_factor_evw.send(SelectFactorEvent { direction });
    }
}

fn handle_mouse_scrolling(mut mouse_scroll: EventReader<MouseWheel>, mut change_scale_evw: EventWriter<ChangeScaleEvent>) {
    for ev in mouse_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                change_scale_evw.send(ChangeScaleEvent(ev.y / 10.))
            },
            MouseScrollUnit::Pixel => {
                change_scale_evw.send(ChangeScaleEvent((ev.y / 3.).round() / 100.))
            }
        }
    }
}

fn apply_scale_factors(
    mut scalable_objects: Query<(&mut Scalable, &mut Transform, &ScaleGroup)>,
    scales: Query<(&ScaleGroup, &Scale)>
) {
    let scales = scales.iter().map(|(g, s)| (*g, s)).collect::<HashMap<ScaleGroup, &Scale>>();
    for (scalable, mut transform, group) in scalable_objects.iter_mut() {
        let scale_group = scales.get(group).unwrap().0;
        let scale = (1. + (scale_group * scalable.factor)).clamp(*scalable.bounds.start(), *scalable.bounds.end());

        let direction_sign = match scalable.direction {
            ScaleDirection::Up | ScaleDirection::Right => 1.,
            ScaleDirection::Down | ScaleDirection::Left => -1.,
        };

        match scalable.direction {
            ScaleDirection::Up | ScaleDirection::Down => {
                transform.scale.y = scale;
                let translation = direction_sign * (scale - 1.) * scalable.size.y / 2.;
                transform.translation.y = scalable.position.y + translation;
            },
            ScaleDirection::Left | ScaleDirection::Right => {
                transform.scale.x = scale;
                let translation = direction_sign * (scale - 1.) * scalable.size.x / 2.;
                transform.translation.x = scalable.position.x + translation;
            }
        }
    }
}

fn update_selection(
    mut selected: ResMut<SelectedGroup>,
    mut evr: EventReader<SelectFactorEvent>,
) {
    let mut selections = ScaleGroup::iter().cycle();

    for ev in evr.read() {
        selected.0 = match ev.direction {
            1 => selections.nth(usize::from(selected.0 as u8 + 1)).unwrap(),
            -1 => selections.nth(usize::from(selected.0 as u8 + ScaleGroup::Blue as u8)).unwrap(),
            _ => unreachable!()
        };
    }
}

fn update_scale(selected: Res<SelectedGroup>, mut q: Query<(&mut Scale, &ScaleGroup)>, mut change_scale_evr: EventReader<ChangeScaleEvent>) {
    let mut scale = q.iter_mut().find(|it| *it.1 == selected.0).unwrap().0;

    for ev in change_scale_evr.read() {
        scale.0 += ev.0;
    }

    scale.0 = scale.0.clamp(-0.9, 4.);
}

fn setup_groups(mut cmd: Commands) {
    cmd.spawn_batch(ScaleGroup::iter().map(|it| (it, Scale(0.))));
    cmd.insert_resource(SelectedGroup(ScaleGroup::Red));
}

fn reset_scales(mut q: Query<&mut Scale, With<ScaleGroup>>) {
    for mut scale in q.iter_mut() {
        scale.0 = 0.;
    }
}

// UI
// TODO: Controls ( Change: Scroll wheel (30px/step))
const UNSELECTED_BG: Color = Color::rgb(0.75, 0.75, 0.75);
const SELECTED_BG: Color = Color::rgb(0.65, 0.65, 0.65);

#[derive(Component)]
pub struct ScaleGroupContainer;

fn setup_ui(mut cmd: Commands) {
    cmd
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(128.),
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                align_self: AlignSelf::End,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            for group in ScaleGroup::iter() {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            border: UiRect::all(Val::Px(2.)),
                            ..Default::default()
                        },
                        background_color: UNSELECTED_BG.into(),
                        ..Default::default()
                    }, 
                    group, ScaleGroupContainer))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(format!("{group:?}: "), TextStyle {
                                font_size: 16.,
                                color: group.into(),
                                ..Default::default()
                            }),
                            Label));
                        parent.spawn((TextBundle::from_section("", TextStyle {
                                font_size: 16.,
                                color: group.into(),
                                ..Default::default()
                        }),
                        Label,
                        group));
                    });
            }
        });
}

fn update_ui_factors(mut q: Query<(&mut Text, &ScaleGroup)>, factors: Query<(&ScaleGroup, &Scale)>) {
    let scales = factors.iter().map(|(g, s)| (*g, s)).collect::<HashMap<ScaleGroup, &Scale>>();

    for (mut text, group) in q.iter_mut() {
        let factor = scales.get(group).unwrap().0;
        text.sections[0].value = format!("{:.2}", factor + 1.);
    }
}

fn update_ui_selected(
    mut q: Query<(&mut BackgroundColor, &ScaleGroup), With<ScaleGroupContainer>>,
    selected_group: Res<SelectedGroup>,
) {
    for (mut bg, group) in q.iter_mut() {
        if *group == selected_group.0 {
            bg.0 = SELECTED_BG;
        } else {
            bg.0 = UNSELECTED_BG;
        }
    }
}


// YOLECK
#[derive(Clone, PartialEq, Serialize, Deserialize, Component, YoleckComponent)]
pub struct YoleckScalable {
    width: Scalar,
    height: Scalar,
    x: Scalar,
    y: Scalar,
    direction: ScaleDirection,
    group: ScaleGroup,
    min: Scalar,
    max: Scalar,
    factor: Scalar,
}

impl Default for YoleckScalable {
    fn default() -> Self {
        Self {
            width: 50.,
            height: 50.,
            x: 0.,
            y: 0.,
            direction: ScaleDirection::Up,
            group: ScaleGroup::Red,
            min: 0.5,
            max: 1.5,
            factor: 1.,
        }
    }
}

fn populate_scalable(mut pop: YoleckPopulate<&YoleckScalable>) {
    pop.populate(|_ctx, mut cmd, scalable| {
        cmd.insert(ScalableBundle::new(
            Scalable {
                factor: scalable.factor,
                direction: scalable.direction,
                position: Vec2::new(scalable.x, scalable.y),
                bounds: scalable.min..=scalable.max,
                size: Vec2::new(scalable.width, scalable.height),
            },
            Vec2::new(scalable.width, scalable.height),
            scalable.group,
        ));
    })
}

fn edit_scalable(mut ui: ResMut<YoleckUi>, mut edit: YoleckEdit<&mut YoleckScalable>) {
    if let Ok(mut scalable) = edit.get_single_mut() {
        egui::ComboBox::from_label("Group")
            .selected_text(format!("{:?}", scalable.group))
            .show_ui(&mut ui, |ui| {
                for variant in ScaleGroup::iter() {
                    ui.selectable_value(&mut scalable.group, variant, format!("{variant:?}"));
                }
            });

        egui::ComboBox::from_label("Direction")
            .selected_text(format!("{:?}", scalable.direction))
            .show_ui(&mut ui, |ui| {
                for variant in ScaleDirection::iter() {
                    ui.selectable_value(&mut scalable.direction, variant, format!("{variant:?}"));
                }
            });

        ui.add(egui::Slider::new(&mut scalable.width, 50.0..=2000.0).prefix("Width: "));
        ui.add(egui::Slider::new(&mut scalable.height, 50.0..=1000.0).prefix("Height: "));

        // TODO: Leave room for scale factor UI
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.add(egui::DragValue::new(&mut scalable.x).speed(1.).fixed_decimals(0).prefix("X: "));

            // TODO: clamp range to height of scale factor UI - INFINITY
            ui.add(egui::DragValue::new(&mut scalable.y).speed(1.).fixed_decimals(0).prefix("Y: "));
        });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.add(egui::Label::new("Scale: "));
            ui.add(egui::DragValue::new(&mut scalable.factor).speed(0.01).fixed_decimals(2).clamp_range(-10.0..=10.).prefix("Factor: "));
            ui.add(egui::DragValue::new(&mut scalable.min).speed(0.1).fixed_decimals(1).prefix("Min: "));
            ui.add(egui::DragValue::new(&mut scalable.max).speed(0.1).fixed_decimals(1).prefix("Max: "));
        });
    }
}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SelectFactorEvent>()
            .add_event::<ChangeScaleEvent>()
            .add_systems(Startup, (setup_groups,setup_ui))
            .add_systems(Update, (
                (handle_keyboard_input, handle_mouse_scrolling).in_set(GameplaySet::Input),
                (update_selection, update_scale).chain().in_set(GameplaySet::Update),
                apply_scale_factors.in_set(GameplaySet::Movement),
                update_ui_factors, update_ui_selected,
            ))
            .add_systems(OnEnter(GameState::LevelEditor), (reset_scales,));

        app.add_yoleck_entity_type(YoleckEntityType::new("Scalable").with::<YoleckScalable>());
        app.add_yoleck_edit_system(edit_scalable);
        app.yoleck_populate_schedule_mut().add_systems(populate_scalable);
    }
}

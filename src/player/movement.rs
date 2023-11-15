//! Handle physics and motion
use bevy::prelude::*;
use bevy_xpbd_2d::{prelude::*, math::*};

use crate::GameplaySet;

// EVENTS
#[derive(Event)]
pub struct MovementEvent(Vector);

// COMPONENTS
#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// A component used to handle player inputs
#[derive(Component)]
pub struct Movement {
    acceleration: Scalar,
    damping_factor: Scalar,
    jump_impulse: Scalar,
    max_slope_angle: Option<Scalar>,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            acceleration: 600.,
            damping_factor: 0.9,
            jump_impulse: 150.,
            max_slope_angle: Some(PI * 0.45),
        }
    }
}

// BUNDLE
#[derive(Bundle)]
pub struct MovementBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    friction: Friction,
    restitution: Restitution,
    movement: Movement,
}

impl MovementBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Vector::NEG_Y)
                .with_max_time_of_impact(10.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            restitution: Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            movement: Movement::default(),
        }
    }
}

// SYSTEMS
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut direction = Vector::ZERO;

    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
    direction.x = (right as i8 - left as i8) as Scalar;

    if keyboard_input.just_pressed(KeyCode::Space) {
        direction.y = 1.0;
    }

    if direction != Vector::ZERO {
        movement_event_writer.send(MovementEvent(direction))
    }
}

fn update_grounded(
    mut cmd: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, &Movement),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, &Movement { max_slope_angle, .. }) in &mut query {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle
            } else {
                true
            }
        });

        if is_grounded {
            cmd.entity(entity).insert(Grounded);
        } else {
            cmd.entity(entity).remove::<Grounded>();
        }
    }
}

fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementEvent>,
    mut controllers: Query<(
        &Movement,
        &mut LinearVelocity,
        Has<Grounded>,
    ), With<CharacterController>>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (&Movement { acceleration, jump_impulse, .. }, mut linear_velocity, is_grounded) in &mut controllers {
            linear_velocity.x += event.0.x * acceleration * delta_time;

            if is_grounded && event.0.y != 0. {
                linear_velocity.y = jump_impulse;
            }
        }
    }
}

fn apply_movement_damping(mut query: Query<(&Movement, &mut LinearVelocity)>) {
    for (Movement { damping_factor, .. }, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor;
    }
}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementEvent>()
            .add_systems(Update, (
                keyboard_input.in_set(GameplaySet::Input),
                update_grounded.after(apply_deferred).in_set(GameplaySet::Update),
                (movement, apply_movement_damping).chain().in_set(GameplaySet::Movement),
            ).chain());
    }
}

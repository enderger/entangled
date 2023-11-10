use bevy::{prelude::*, render::camera::ScalingMode};

use crate::player::Player;

pub const WINDOW_SIZE: Vec2 = Vec2::new(1024., 720.);
pub const WINDOW_BOTTOM_LEFT: Vec2 = Vec2::new(WINDOW_SIZE.x / -2., WINDOW_SIZE.y / -2.);



// SYSTEMS
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -100.,
            scaling_mode: ScalingMode::Fixed {
                width: WINDOW_SIZE.x, 
                height: WINDOW_SIZE.y,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn move_camera_game(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    let Ok(mut camera) = camera_query.get_single_mut() else { return };
    let Ok(player) = player_query.get_single() else { return };

    camera.translation = Vec3::new(f32::max(player.translation.x, 0.), f32::max(player.translation.y, 0.), camera.translation.z);
}

fn move_camera_editor(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);

    let motion = Vec2::new((right as i8 - left as i8) as f32, (up as i8 - down as i8) as f32) * 2.;
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };
    camera_transform.translation += motion.extend(0.);

    if camera_transform.translation.x <= 0. {
        camera_transform.translation.x = 0.;
    }

    if camera_transform.translation.y <= 0. {
        camera_transform.translation.y = 0.;
    }
}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, (setup_camera,))
            .add_systems(Update, (move_camera_game.in_set(crate::GameplaySet::Update),))
            .add_systems(Update, (move_camera_editor.in_set(crate::EditorSet),));
    }
}

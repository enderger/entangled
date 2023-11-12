//! Death & Respawn logic
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::GameplaySet;

use super::Player;

// EVENTS
#[derive(Event)]
pub struct Respawn;

// SYSTEMS
pub fn check_out_of_bounds(q: Query<&Transform, With<Player>>, mut evw: EventWriter<Respawn>) {
    q.for_each(|transform| {
        if transform.translation.y < crate::camera::WINDOW_BOTTOM_LEFT.y {
            evw.send(Respawn)
        }
    })
}

// TODO: Check if player is squished

pub fn respawn(mut q: Query<(&mut Transform, &mut LinearVelocity), With<Player>>, mut evr: EventReader<Respawn>) {
    let Ok(mut player) = q.get_single_mut() else { return };

    if evr.read().last().is_some() {
        player.0.translation.x = crate::camera::WINDOW_BOTTOM_LEFT.x + 150.;
        player.0.translation.y = -100.0;
        player.1.0 = Vec2::ZERO;
    }
}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Respawn>()
            .add_systems(Update, (
                check_out_of_bounds.in_set(GameplaySet::Update),
                respawn.in_set(GameplaySet::Movement)
            ));
    }
}

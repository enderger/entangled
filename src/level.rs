// TODO: level system w/ YOLECK
use bevy::prelude::*;

// SUBMODULES
pub mod platform;
pub mod resizable;

// SYSTEMS
///// Set up the basic test level (TODO: levels with editor)
//pub fn setup(mut cmd: Commands) {
//    cmd.spawn(PlatformBundle::new(Vec2::new(-350.0, crate::WINDOW_BOTTOM_LEFT.y + 75. / 2.), Vec2::new(200., 75.)));
//    cmd.spawn(PlatformBundle::new(Vec2::new(100.0, crate::WINDOW_BOTTOM_LEFT.y + 175.), Vec2::new(50., 350.)));
//    cmd.spawn(PlatformBundle::new(Vec2::new(350.0, crate::WINDOW_BOTTOM_LEFT.y + 125.), Vec2::new(150., 250.)));
//}

// PLUGIN
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((platform::Plugin, resizable::Plugin))
            /*.add_systems(Startup, (setup,))*/;
    }
}

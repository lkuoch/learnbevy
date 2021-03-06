use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .commands()
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

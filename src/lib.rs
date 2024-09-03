use bevy::prelude::*;

#[allow(dead_code)]
pub fn close_on_esc(mut commands: Commands, windows: Query<(Entity, &Window)>, input: Res<ButtonInput<KeyCode>>) {
    for (window, focus) in windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

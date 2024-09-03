#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(special_module_name)]

mod lib;

use bevy::{prelude::*, window::WindowResolution};
use grey_bevy_experiments::close_on_esc;

// I'm a sucker for Solarized Dark
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 43.0 / 255.0, 54.0 / 255.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Experiments".to_string(),
                resolution: WindowResolution::new(1200.0, 600.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Msaa::Sample4) // TODO: Ensure I actually understand this.
        .add_systems(Startup, (setup_camera, setup_sprites))
        .add_systems(Update, close_on_esc)
        .add_systems(Update, (animate_sprites, move_player))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        camera: Camera { hdr: true, ..default() },
        ..default()
    },));
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_sprites(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = assets.load("sprites/mio run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(51, 70), 2, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteBundle { texture, ..default() },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: indices.first,
        },
        indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

const PLAYER_SPEED: f32 = 200.0;

#[derive(Component)]
struct Player;

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    mut sprite: Query<&mut Sprite, With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let Ok(mut sprite) = sprite.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;

        sprite.flip_x = true;
    }

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;

        sprite.flip_x = false;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player.translation += move_delta.extend(0.0);
}

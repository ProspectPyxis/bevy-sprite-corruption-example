use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_camera)
                .with_system(spawn_player),
        )
        .add_system(player_shoot)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/player.png"),
        // texture: asset_server.load("textures/simplespace/enemy_A.png"),
        ..Default::default()
    });
}

pub fn player_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/fireball.png"),
            // texture: asset_server.load("textures/simplespace/enemy_B.png"),
            transform: Transform {
                translation: Vec3::new(
                    rand::thread_rng().gen_range(-640.0..640.0),
                    rand::thread_rng().gen_range(-480.0..480.0),
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

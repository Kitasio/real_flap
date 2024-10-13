use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_background);
    }
}

fn add_background(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    windows: Query<&Window, With<Window>>,
) {
    let window = windows.single();
    let width = window.width();
    let height = window.height();

    let texture = scene_assets.bg.clone();
    let sprite = Sprite {
        custom_size: Some(Vec2::new(width, height)),
        ..default()
    };
    commands
        .spawn(SpriteBundle {
            texture,
            sprite,
            ..default()
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        }));
}

// fn insert_screen_boundaries(mut commands: Commands, windows: Query<&Window, With<Window>>) {
//     let window = windows.single();
//     let width = window.width();
//     let height = window.height();
//
//     commands
//         .spawn(Collider::cuboid(width / 2.0, height / 2.0)) // Collider is centered, so halve the size
//         .insert(TransformBundle::from(Transform::from_xyz(
//             width / 2.0,
//             height / 2.0,
//             0.0, // Offset to align the cuboid with screen bounds
//         )));
// }

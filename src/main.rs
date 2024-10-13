use crate::state::GameState;
use asset_loader::AssetLoaderPlugin;
use background::BackgroundPlugin;
use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use obstacle::ObstaclePlugin;
use player::PlayerPlugin;

mod asset_loader;
mod background;
mod camera;
mod collision_detection;
mod obstacle;
mod player;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(GameState::NotStarted)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(BackgroundPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ObstaclePlugin)
        .add_plugins(CollisionDetectionPlugin)
        .run();
}

// fn print_ball_altitude(positions: Query<&Transform, With<Ball>>) {
//     for transform in positions.iter() {
//         println!("Position: {}", transform.translation.y);
//     }
// }
//

// fn display_events(
//     mut collision_events: EventReader<CollisionEvent>,
//     mut contact_force_events: EventReader<ContactForceEvent>,
// ) {
//     for collision_event in collision_events.read() {
//         println!("Received collision event: {:?}", collision_event);
//     }
//
//     for contact_force_event in contact_force_events.read() {
//         println!("Received contact force event: {:?}", contact_force_event);
//     }
// }

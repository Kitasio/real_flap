use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::Collider;

use crate::{asset_loader::SceneAssets, state::GameState};

const SPAWN_TIME_SECONDS: f32 = 2.5;

#[derive(Component, Debug)]
struct Obstacle;

#[derive(Resource, Debug)]
struct SpawnTimer {
    timer: Timer,
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        // .add_systems(Startup, insert_screen_boundaries)
        .add_systems(Update, spawn_obstacles.run_if(in_state(GameState::Started)))
        .add_systems(Update, move_obstacles.run_if(in_state(GameState::Started)));
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    windows: Query<&Window, With<Window>>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }
    let window = windows.single();
    let height = window.height();

    let gap_size = 285.0;
    let obstacle_width = 100.0;
    let obstacle_height = 1000.0;
    let screen_height = height / 2.0 - gap_size / 2.0 - obstacle_height / 2.0;

    let gap_center = rand::random::<f32>() * (screen_height - gap_size) + gap_size / 4.0;

    let upper_obstacle_y = gap_center + gap_size / 2.0 + obstacle_height / 2.0;
    let lower_obstacle_y = gap_center - gap_size / 2.0 - obstacle_height / 2.0;

    let upper_obstacle: Handle<Image>;
    let upper_sprite: Sprite;

    let lower_obstacle: Handle<Image>;
    let lower_sprite: Sprite;

    if rand::random() {
        upper_obstacle = scene_assets.hotdog_obstacle.clone();
        upper_sprite = Sprite {
            custom_size: Some(Vec2::new(400.0, 1000.0)),
            anchor: Anchor::Custom(Vec2::new(0.0, 0.19)),
            flip_y: true,
            ..default()
        };
        lower_obstacle = scene_assets.hotdog_obstacle.clone();
        lower_sprite = Sprite {
            custom_size: Some(Vec2::new(400.0, 1000.0)),
            anchor: Anchor::Custom(Vec2::new(0.0, -0.19)),
            ..default()
        };
    } else {
        upper_obstacle = scene_assets.money_obstacle.clone();
        upper_sprite = Sprite {
            custom_size: Some(Vec2::new(400.0, 1000.0)),
            anchor: Anchor::Custom(Vec2::new(0.0, 0.08)),
            flip_y: true,
            ..default()
        };
        lower_obstacle = scene_assets.money_obstacle.clone();
        lower_sprite = Sprite {
            custom_size: Some(Vec2::new(400.0, 1000.0)),
            anchor: Anchor::Custom(Vec2::new(0.0, -0.08)),
            ..default()
        };
    }

    commands
        .spawn(Obstacle)
        .insert(SpriteBundle {
            texture: upper_obstacle,
            sprite: upper_sprite,
            ..default()
        })
        .insert(Collider::cuboid(
            obstacle_width / 2.0,
            obstacle_height / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            2000.0,
            upper_obstacle_y,
            0.0,
        )));

    commands
        .spawn(Obstacle)
        .insert(SpriteBundle {
            texture: lower_obstacle,
            sprite: lower_sprite,
            ..default()
        })
        .insert(Collider::cuboid(
            obstacle_width / 2.0,
            obstacle_height / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            2000.0,
            lower_obstacle_y,
            0.0,
        )));
}

fn move_obstacles(time: Res<Time>, mut query: Query<(&mut Transform, &Obstacle)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.x += -200.0 * time.delta_seconds();
    }
}

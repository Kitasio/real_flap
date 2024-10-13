use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Restitution, RigidBody, Velocity};

use crate::{asset_loader::SceneAssets, GameState};

#[derive(Component, Debug)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_player)
            .add_systems(
                Update,
                start_player_movement.run_if(in_state(GameState::NotStarted)),
            )
            .add_systems(Update, move_player.run_if(in_state(GameState::Started)))
            .add_systems(
                Update,
                toggle_image_on_jump.run_if(in_state(GameState::Started)),
            );
    }
}

fn add_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let texture = scene_assets.lungi.clone();
    let sprite = Sprite {
        custom_size: Some(Vec2::new(150.0, 150.0)),
        ..default()
    };
    /* Create a floating ball. */
    commands
        .spawn(Player)
        .insert(SpriteBundle {
            texture,
            sprite,
            ..default()
        })
        .insert(Collider::ball(50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn start_player_movement(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &Player)>,
    scene_assets: Res<SceneAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Started);

        // Remove the floating ball
        let (floating_ball_id, _) = query.single_mut();
        commands.entity(floating_ball_id).despawn();

        let texture = scene_assets.lungi.clone();
        let sprite = Sprite {
            custom_size: Some(Vec2::new(150.0, 150.0)),
            ..default()
        };

        /* Create the bouncing ball. */
        commands
            .spawn(Player)
            .insert(SpriteBundle {
                texture,
                sprite,
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(50.0))
            .insert(Velocity {
                linvel: Vec2::new(0.0, 500.0),
                ..Default::default()
            })
            .insert(Restitution::coefficient(0.9))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ActiveEvents::CONTACT_FORCE_EVENTS);
    }
}

fn toggle_image_on_jump(
    mut query: Query<(&Player, &Velocity, &mut Handle<Image>)>,
    scene_assets: Res<SceneAssets>,
) {
    let (_player, velocity, mut image) = query.single_mut();
    if velocity.linvel.y > 0.0 {
        *image = scene_assets.lungi_jumps.clone();
    } else {
        *image = scene_assets.lungi.clone();
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity)>,
) {
    let (_player, mut velocity) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.linvel.y = 500.0;
    }
}

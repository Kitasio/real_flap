use bevy::prelude::*;
use bevy_rapier2d::prelude::ContactForceEvent;

use crate::state::GameState;

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_collisions);
    }
}

fn check_for_collisions(
    mut contact_force_events: EventReader<ContactForceEvent>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for contact_force_event in contact_force_events.read() {
        println!("Contact force event: {:?}", contact_force_event);

        if state.get() == &GameState::Started {
            next_state.set(GameState::GameOver);
        }

        info!("Game Over! - state: {:?}", state.get());
    }
}

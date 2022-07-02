use super::GameState;
use super::MultiplayerState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod networking; // TODO: move connecting/disconnecting to a higher level module so this can be private
use networking::lobby_network_system;

mod ui;
use ui::*;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<UiState>()
        .init_resource::<MultiplayerState>() // This should be moved to a more generic location
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(lobby_ui)
                .with_system(lobby_network_system
                    .run_if_resource_exists::<MultiplayerState>())
                .into()
        );
    }
}
use bevy::prelude::*;

/// Marker component for the player entity.
#[derive(Component)]
pub struct Player;

/// Placeholder system – spawns a player entity (no mesh yet).
pub fn setup(mut commands: Commands) {
    commands.spawn((Player, Name::new("Player")));
}

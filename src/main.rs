mod player;
mod world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(
            Startup,
            (setup_camera, world::setup, player::setup, setup_physics),
        )
        .run();
}

/// Spawns the 3-D perspective camera used throughout the game.
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Initialises physics bodies: a static ground plane and a dynamic ball that
/// falls under gravity.
fn setup_physics(mut commands: Commands) {
    // Static ground collider – a flat cuboid that acts as the floor.
    commands.spawn((
        Collider::cuboid(10.0, 0.1, 10.0),
        RigidBody::Fixed,
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));

    // Dynamic rigid body – a sphere that falls and bounces on the ground.
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Restitution::coefficient(0.7),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));
}

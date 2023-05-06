//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

mod setup;
mod player;

use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use player::PlayerPlugin;
use setup::SetupPlugin;

const GRAVITY_CONSTANT: Vec3 = Vec3::new(0.0, -980.0, 0.0);
const PLAYER_SIZE: f32 = 100.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.8, 0.4)))
        .run();
}
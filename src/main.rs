use bevy::{prelude::*, render::texture::ImageSampler};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 1080.0,
                    height: 720.0,
                    title: "To do".to_string(),
                    resizable: true,
                    ..Default::default()
                },
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: ImageSampler::nearest_descriptor(),
        }))
        .add_startup_system(setup_graphics)
        .run();
}

fn setup_graphics(mut commands: Commands) {

    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    
}

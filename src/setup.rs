use bevy_rapier3d::prelude::*;

use bevy::{
    prelude::*,
    render::{render_resource::{Extent3d, TextureDimension, TextureFormat}, camera::Viewport}, core_pipeline::clear_color::ClearColorConfig, window::{WindowResized, WindowId},
};

use crate::{player::Player, PLAYER_SIZE};
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_startup_system(setup)
            // .add_system(set_camera_viewports)
            ;
    }
}


// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct GlobalCamera;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    
    let camera = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, PLAYER_SIZE * 4.0, PLAYER_SIZE * 4.0).looking_at(Vec3::new(0.0, 0.0, -100.0), Vec3::Y),
        camera: Camera {
            priority: 2,
            ..Default::default()
        },
        ..default()
    }).insert(PlayerCamera).id();

    let mut player = shape::Capsule::default();
    (player.radius, player.depth) = (PLAYER_SIZE / 2.0, PLAYER_SIZE);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(player.into()),
            material: debug_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                ..default()
            },
            ..default()
        },
        Shape,
        KinematicCharacterController::default(),
        KinematicCharacterControllerOutput::default(),
        Collider::capsule_y(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0), 
        Player { run_speed: 300.0, velocity: Vec3::ZERO, jump_velocity: 600.0 },       
    )).push_children(&[camera]);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000000.0,
            range: 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 1000.0, 0.0),
        ..default()
    });

    // ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane{size: 10000.0}.into()),
            material: materials.add(Color::YELLOW.into()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5000.0, 0.0, 5000.0)
    ));

    // global camera

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3000.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            camera: Camera {
                // Renders the right camera after the player camera, which has a default priority of 0
                priority: 1,
                ..default()
            },
            camera_3d: Camera3d {
                // don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        GlobalCamera,
    ));

}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

// fn set_camera_viewports(
//     windows: Res<Windows>,
//     mut resize_events: EventReader<WindowResized>,
//     mut left_camera: Query<&mut Camera, (With<PlayerCamera>, Without<GlobalCamera>)>,
//     mut right_camera: Query<&mut Camera, With<GlobalCamera>>,
// ) {
//     // We need to dynamically resize the camera's viewports whenever the window size changes
//     // so then each camera always takes up half the screen.
//     // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
//     for resize_event in resize_events.iter() {
//         if resize_event.id == WindowId::primary() {
//             let window = windows.primary();
//             let mut left_camera = left_camera.single_mut();
//             left_camera.viewport = Some(Viewport {
//                 physical_position: UVec2::new(0, 0),
//                 physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
//                 ..default()
//             });

//             let mut right_camera = right_camera.single_mut();
//             right_camera.viewport = Some(Viewport {
//                 physical_position: UVec2::new(window.physical_width() / 2, 0),
//                 physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
//                 ..default()
//             });
//         }
//     }
// }
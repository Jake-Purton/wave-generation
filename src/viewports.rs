use bevy::{prelude::*, window::{WindowResized, WindowId}, render::camera::Viewport};

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct GlobalCamera;

pub struct CameraViewportPlugin;

impl Plugin for CameraViewportPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_system(set_camera_viewports);
    }
}

fn set_camera_viewports(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera: Query<&mut Camera, (With<PlayerCamera>, Without<GlobalCamera>)>,
    mut right_camera: Query<&mut Camera, With<GlobalCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();
            let mut left_camera = left_camera.single_mut();
            left_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                ..default()
            });

            let mut right_camera = right_camera.single_mut();
            right_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(window.physical_width() / 2, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                ..default()
            });
        }
    }
}
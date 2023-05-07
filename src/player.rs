use bevy::input::mouse::MouseMotion;
use ::bevy::prelude::*;

use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{GRAVITY_CONSTANT, viewports::{PlayerCamera, GlobalCamera}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(rapier_player_movement);
    }
}

#[derive(Component)]
pub struct Player {
    pub run_speed: f32,
    pub velocity: Vec3,
    pub jump_velocity: f32,
}

pub fn rapier_player_movement (
    mut controllers: Query<(
        &mut KinematicCharacterController, 
        &mut Player, 
        &KinematicCharacterControllerOutput,
        &mut Transform,
    )>,
    mut player_camera: Query<
        &mut Transform,
        (
            With<PlayerCamera>,
            Without<GlobalCamera>,
            Without<Player>
        )   
    >,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
) {

    for (mut controller, mut player, output, mut transform) in controllers.iter_mut() {

        let delta_s = time.delta_seconds();

        let mut movement = Vec3::ZERO;

        if player.velocity.x.is_nan() {
            player.velocity.x = 0.0;
        } 
        if player.velocity.y.is_nan() {
            player.velocity.y = 0.0;
        }

        // make sure it hits the ceiling
        if output.effective_translation.y.is_sign_positive() && (output.effective_translation.y * 10.0).round() == 0.0 {
            player.velocity.y = 0.0;
        }

        player_camera.single_mut();
        let mouse_movement: Vec2 = motion_evr.iter().map(|a| a.delta).sum();

        // anfinbd the angle of camera to player
        // move the camera around the player ig

        if keys.pressed(KeyCode::W) {
            movement += Vec3::new(0.0, 0.0, player.run_speed);
        }        
        if keys.pressed(KeyCode::S) {
            movement += Vec3::new(0.0, 0.0, -player.run_speed);
        }        
        if keys.pressed(KeyCode::D) {
            movement += Vec3::new(player.run_speed, 0.0, 0.0);
        }        
        if keys.pressed(KeyCode::A) {
            movement += Vec3::new(-player.run_speed, 0.0, 0.0);
        }        

        if !output.grounded {
            player.velocity += GRAVITY_CONSTANT * delta_s;
        } else {

            player.velocity.x = 0.0;

            if keys.pressed(KeyCode::Space) {
                player.velocity.y = player.jump_velocity;
            } else {
                player.velocity.y = 0.0;
            }
        }

        movement += player.velocity;

        controller.translation = Some(movement * delta_s);

        if keys.just_pressed(KeyCode::F) {
            println!("velocity: {}, movement: {} ", player.velocity, movement);
        }
    }
}
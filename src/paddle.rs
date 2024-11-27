use bevy::prelude::*;

use crate::WindowDimensions;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (move_paddle, update_to_screensize));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let paddle_texture_handle = asset_server.load("paddle.png");
    let paddle_dimensions = PaddleDimensions::default();

    commands.spawn((
        Paddle(0),
        SpriteBundle {
            texture: paddle_texture_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(paddle_dimensions.width, paddle_dimensions.height, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    commands.spawn((
        Paddle(1),
        SpriteBundle {
            texture: paddle_texture_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(paddle_dimensions.width, paddle_dimensions.height, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    commands.insert_resource(paddle_dimensions);
}

const PADDLE_OFFSET_FROM_WALL: f32 = 50.0;
fn update_to_screensize(
    window_dimension: Res<WindowDimensions>,
    mut paddles: Query<(&mut Transform, &Paddle)>,
) {
    for (mut transform, paddle) in paddles.iter_mut() {
        let offset = if paddle.0 == 0 {
            (window_dimension.width / 2.0) - PADDLE_OFFSET_FROM_WALL
        } else {
            -(window_dimension.width / 2.0) + PADDLE_OFFSET_FROM_WALL
        };

        transform.translation.x = offset;
    }
}

fn move_paddle(
    window_dimension: Res<WindowDimensions>,
    mut paddles: Query<(&mut Transform, &Paddle)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, paddle) in paddles.iter_mut() {
        let key_codes = if paddle.0 == 0 {
            (KeyCode::ArrowUp, KeyCode::ArrowDown)
        } else {
            (KeyCode::KeyW, KeyCode::KeyS)
        };

        // Up
        if keys.pressed(key_codes.0) {
            transform.translation.y += window_dimension.height * 0.75 * time.delta_seconds();
        };

        // Down
        if keys.pressed(key_codes.1) {
            transform.translation.y -= window_dimension.height * 0.75 * time.delta_seconds();
        };

        let half_height = PADDLE_HEIGHT / 2.0;
        let half_window_height = window_dimension.height / 2.0;
        // Check boundaries
        if transform.translation.y + half_height > half_window_height {
            transform.translation.y = half_window_height - half_height;
            info!("Paddle hit upper bound");
        } else if transform.translation.y - half_height < -half_window_height {
            transform.translation.y = -half_window_height + half_height;
            info!("Paddle hit lower bound");
        };
    }
}

#[derive(Debug, Component)]
pub struct Paddle(u8);

pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 160.0;

#[derive(Debug, Resource)]
pub struct PaddleDimensions {
    pub width: f32,
    pub height: f32,
}

impl Default for PaddleDimensions {
    fn default() -> Self {
        Self {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

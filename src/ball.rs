use bevy::prelude::*;

use crate::{
    paddle::{Paddle, PaddleDimensions},
    Score, Velocity, WindowDimensions,
};

const BASE_BALL_SPEED: f32 = 300.0;
const BALL_SPEED_MULTIPLIER_INCREASE_PER_SECOND: f32 = 0.0001;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                wait_for_ball_assets.before(update_ball_speed_multiplier),
                update_ball_speed_multiplier.before(update_ball),
                update_ball,
                check_for_wall_bounce.after(update_ball),
                check_for_point.after(update_ball),
                check_for_paddle_collision.after(update_ball),
            ),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    let ball_texture_handle = asset_server.load("ball.png");

    commands.insert_resource(BallAssetImageId(ball_texture_handle.id()));
    commands.insert_resource(BallImageSize(None));

    commands.insert_resource(BallSpeedMultiplier(1.0));

    commands.spawn((
        Ball,
        Velocity::from_random(&mut rng),
        IsBallColliding::default(),
        SpriteBundle {
            texture: ball_texture_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

// FIXME: This is yank and needs a different solution. Sadly I don't know better.
fn wait_for_ball_assets(
    images: Res<Assets<Image>>,
    ball_asset_image_id: Res<BallAssetImageId>,
    mut ball_image_size: ResMut<BallImageSize>,
    asset_server: Res<AssetServer>,
) {
    use bevy::asset::LoadState;

    if ball_image_size.0.is_none() {
        match asset_server.load_state(ball_asset_image_id.0) {
            LoadState::Failed(e) => {
                error!("Could not load ball image! Error: {}", e);
                panic!("Could not load ball image! Error: {}", e);
            }
            LoadState::Loaded => {
                let ball_size = match images.get(ball_asset_image_id.0).map(|image| image.size()) {
                    Some(ball_size) => Vec2::new(ball_size.x as f32, ball_size.y as f32),
                    None => panic!("Could not get image size!"),
                };

                ball_image_size.0 = Some(ball_size);
            }
            _ => (),
        }
    }
}

fn update_ball(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Ball>>,
    ball_speed_multiplier: Res<BallSpeedMultiplier>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x +=
            velocity.x * BASE_BALL_SPEED * ball_speed_multiplier.0 * time.delta_seconds();
        transform.translation.y +=
            velocity.y * BASE_BALL_SPEED * ball_speed_multiplier.0 * time.delta_seconds();
    }
}

fn check_for_wall_bounce(
    window_dimensions: Res<WindowDimensions>,
    mut query: Query<(&Transform, &mut Velocity, &mut IsBallColliding), With<Ball>>,
    image_size: Res<BallImageSize>,
) {
    let half_image_size = match image_size.0 {
        Some(image_size) => Vec2::new(image_size.x / 2.0, image_size.y / 2.0),
        None => Vec2::new(0.0, 0.0),
    };

    let height = window_dimensions.height / 2.0;

    let (transform, mut velocity, mut collided) = query.single_mut();

    if transform.translation.y - half_image_size.x <= -height
        || transform.translation.y + half_image_size.x >= height
    {
        if !collided.wall {
            info!("Ball hit wall");
            velocity.y *= -1.0;

            collided.wall = true;
        }
    } else {
        collided.wall = false;
    };
}

fn check_for_point(
    window_dimensions: Res<WindowDimensions>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    image_size: Res<BallImageSize>,
    mut score: ResMut<Score>,
) {
    let half_image_size = match image_size.0 {
        Some(image_size) => Vec2::new(image_size.x / 2.0, image_size.y / 2.0),
        None => Vec2::new(0.0, 0.0),
    };

    let width = window_dimensions.width / 2.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        if transform.translation.x - half_image_size.x <= -width {
            info!("Point player 1");
            reset(&mut transform, &mut velocity);
            score.p1 += 1;
        } else if transform.translation.x + half_image_size.x >= width {
            info!("Point player 2");
            reset(&mut transform, &mut velocity);
            score.p2 += 1;
        };
    }
}

fn check_for_paddle_collision(
    paddle_dimensions: Res<PaddleDimensions>,
    mut balls: Query<(&mut Velocity, &Transform, &mut IsBallColliding), With<Ball>>,
    paddles: Query<&Transform, With<Paddle>>,
    image_size: Res<BallImageSize>,
) {
    let mut just_collided = false;

    let (mut ball_velocity, ball_transform, mut collided) = balls.single_mut();

    for paddle_transform in paddles.iter() {
        if collide_ball_paddle(
            ball_transform,
            paddle_transform,
            &paddle_dimensions,
            &image_size,
        ) {
            if !collided.paddle {
                info!("Collided!");

                ball_velocity.x *= -1.0;

                collided.paddle = true;
            }

            just_collided = true;
        }
    }

    if !just_collided {
        collided.paddle = false;
    }
}

fn collide_ball_paddle(
    ball_transform: &Transform,
    paddle_transform: &Transform,
    paddle_dimensions: &PaddleDimensions,
    image_size: &Res<BallImageSize>,
) -> bool {
    let image_size = match image_size.0 {
        Some(image_size) => image_size,
        None => Vec2::new(0.0, 0.0),
    };

    let half_paddle_size_x = paddle_dimensions.width / 2.0;
    let half_paddle_size_y = paddle_dimensions.height / 2.0;
    let half_ball_size = image_size.x / 2.0;

    let collision_x = (paddle_transform.translation.x - half_paddle_size_x
        < ball_transform.translation.x + half_ball_size)
        && (paddle_transform.translation.x + half_paddle_size_x
            > ball_transform.translation.x - half_ball_size);

    let collision_y = (paddle_transform.translation.y - half_paddle_size_y
        < ball_transform.translation.y + half_ball_size)
        && (paddle_transform.translation.y + half_paddle_size_y
            > ball_transform.translation.y - half_ball_size);

    collision_x && collision_y
}

pub fn reset(transform: &mut Transform, velocity: &mut Velocity) {
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;

    let mut rng = rand::thread_rng();
    velocity.regen(&mut rng);
}

fn update_ball_speed_multiplier(
    time: Res<Time>,
    mut ball_speed_multiplier: ResMut<BallSpeedMultiplier>,
) {
    ball_speed_multiplier.0 += BALL_SPEED_MULTIPLIER_INCREASE_PER_SECOND * time.delta_seconds();
    info!("Speed Multiplier: {}", ball_speed_multiplier.0);
}

#[derive(Debug, Component)]
struct Ball;

#[derive(Debug, Default, Component)]
struct IsBallColliding {
    paddle: bool,
    wall: bool,
}

#[derive(Debug, Resource)]
struct BallImageSize(Option<Vec2>);

#[derive(Debug, Resource)]
struct BallAssetImageId(AssetId<Image>);

#[derive(Debug, Resource)]
struct BallSpeedMultiplier(f32);

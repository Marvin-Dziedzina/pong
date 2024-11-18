use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

const BALL_RADIUS: f32 = 25.0;
const BALL_SPEED: f32 = 450.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (update_ball, check_for_bounce, check_for_point).chain(),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let mut rng = rand::thread_rng();

    let window = windows.single();
    commands.insert_resource(WindowDimensions {
        width: window.width(),
        height: window.height(),
    });

    commands.spawn((
        Ball,
        Velocity(Vec2::new(
            rng.gen_range(-2.0..1.0) + 1.0,
            rng.gen_range(-0.5..0.5),
        )),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_RADIUS))),
            material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}

fn update_ball(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0 = velocity.0.normalize();

        transform.translation.x += velocity.0.x * BALL_SPEED * time.delta_seconds();
        transform.translation.y += velocity.0.y * BALL_SPEED * time.delta_seconds();
    }
}

fn check_for_bounce(
    window_dimensions: Res<WindowDimensions>,
    mut query: Query<(&Transform, &mut Velocity), With<Ball>>,
) {
    let height = window_dimensions.height / 2.0;

    for (transform, mut velocity) in query.iter_mut() {
        if transform.translation.y - BALL_RADIUS <= -height
            || transform.translation.y + BALL_RADIUS >= height
        {
            info!("Ball hit wall");
            velocity.0.y *= -1.0;
        };
    }
}

fn check_for_point(
    window_dimensions: Res<WindowDimensions>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let width = window_dimensions.width / 2.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        if transform.translation.x - BALL_RADIUS <= -width
            || transform.translation.x + BALL_RADIUS >= width
        {
            info!("Point");

            reset(&mut transform, &mut velocity);
        };
    }
}

pub fn reset(transform: &mut Transform, velocity: &mut Velocity) {
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;

    let mut rng = rand::thread_rng();
    velocity.0.x += rng.gen_range(-1.0..1.0);
    velocity.0.y += rng.gen_range(-0.5..0.5);
}

#[derive(Debug, Component)]
struct Ball;

#[derive(Debug, Component)]
struct Velocity(Vec2);

#[derive(Debug, Resource)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
}

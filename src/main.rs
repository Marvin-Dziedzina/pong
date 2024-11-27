use bevy::{prelude::*, window::WindowResized};

mod ball;
mod paddle;

use ball::BallPlugin;
use paddle::PaddlePlugin;
use rand::{rngs::ThreadRng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BallPlugin)
        .add_plugins(PaddlePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_window_dimensions)
        .init_resource::<WindowDimensions>()
        .run();
}

fn setup(mut commands: Commands, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single();
    commands.insert_resource(WindowDimensions {
        width: window.width(),
        height: window.height(),
    });
}

fn update_window_dimensions(
    mut window_dimensions: ResMut<WindowDimensions>,
    resize_event: Res<Events<WindowResized>>,
) {
    let mut reader = resize_event.get_reader();
    for event in reader.read(&resize_event) {
        window_dimensions.width = event.width;
        window_dimensions.height = event.height;
    }
}

#[derive(Debug, Default, Resource)]
struct WindowDimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Component, Default)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    pub fn from_random(rng: &mut ThreadRng) -> Self {
        Self {
            x: Self::gen_direction(rng),
            y: Self::gen_direction(rng),
        }
    }

    pub fn regen(&mut self, rng: &mut ThreadRng) {
        self.x = Self::gen_direction(rng);
        self.y = Self::gen_direction(rng);
    }

    fn gen_direction(rng: &mut ThreadRng) -> f32 {
        if rng.gen_bool(0.5) {
            1.0
        } else {
            -1.0
        }
    }
}

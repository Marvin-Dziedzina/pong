use bevy::{prelude::*, window::WindowResized};

mod ball;
mod paddle;

use ball::BallPlugin;
use paddle::PaddlePlugin;
use rand::{rngs::ThreadRng, Rng};

const SCORE_SIZE: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BallPlugin)
        .add_plugins(PaddlePlugin)
        .init_resource::<WindowDimensions>()
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_window_dimensions)
        .add_systems(Update, (update_score_position, update_score_text))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single();
    commands.insert_resource(WindowDimensions {
        width: window.width(),
        height: window.height(),
    });

    let font = asset_server.load("fonts/Tenorite.ttf");

    commands.spawn((
        PlayerId(0),
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: font.clone(),
                    font_size: 1.0,
                    ..Default::default()
                },
            ),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale: Vec3::new(SCORE_SIZE, SCORE_SIZE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    commands.spawn((
        PlayerId(1),
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font,
                    font_size: 1.0,
                    ..Default::default()
                },
            ),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale: Vec3::new(SCORE_SIZE, SCORE_SIZE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
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

#[derive(Debug, Resource, Default)]
struct Score {
    pub p1: u32,
    pub p2: u32,
}

#[derive(Debug, Component)]
struct PlayerId(u8);

fn update_score_position(
    mut score_texts: Query<(&mut Transform, &PlayerId), With<Text>>,
    window_dimensions: Res<WindowDimensions>,
) {
    for (mut transform, player_id) in score_texts.iter_mut() {
        transform.translation.x = (window_dimensions.width / 2.0) / 2.0;
        transform.translation.y = (window_dimensions.height / 2.0) - 50.0;

        if player_id.0 == 1 {
            transform.translation.x *= -1.0;
        };
    }
}

fn update_score_text(mut score_texts: Query<(&mut Text, &PlayerId)>, score: Res<Score>) {
    for (mut text, player_id) in score_texts.iter_mut() {
        if player_id.0 == 0 {
            text.sections.clear();
            text.sections.push(TextSection::from(score.p1.to_string()));
        } else if player_id.0 == 1 {
            text.sections.clear();
            text.sections.push(TextSection::from(score.p2.to_string()));
        }
    }
}

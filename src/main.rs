use bevy::prelude::*;

const GRAVITY: f32 = -0.4;

const WINDOW_WIDTH: f32 = 400.;
const WINDOW_HEIGHT: f32 = 500.;

const BIRD_RADIUS: f32 = 20.;
const BIRD_FLAP: f32 = 9.;
const BIRD_START: f32 = 100.;

const FLY_GAP: f32 = 150.;

const SCROLL_SPEED: f32 = 5.;

const PIPE_START: f32 = 600.;
const PIPE_SEP: f32 = 300.;
const PIPE_WIDTH: f32 = 50.;
const PIPE_HEIGHT: f32 = 400.;
const PIPE_BASE_Y: f32 = (PIPE_HEIGHT / 2.) + (FLY_GAP / 2.);

#[derive(Component, Default)]
struct Bird {
    velocity: f32,
}

#[derive(Component, Default)]
struct Pipe {
    y_offset: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Flopperbird"),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (gravity, flap, move_pipes).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Bird::default(),
        Mesh2d(meshes.add(Circle::new(BIRD_RADIUS))),
        MeshMaterial2d(materials.add(Color::srgb(1., 0., 1.))),
        Transform::from_xyz(0., BIRD_START, 1.),
    ));

    // First set
    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(PIPE_WIDTH, PIPE_HEIGHT))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(PIPE_START, PIPE_BASE_Y, 0.),
    ));

    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(PIPE_WIDTH, PIPE_HEIGHT))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(PIPE_START, -PIPE_BASE_Y, 0.),
    ));

    // Second set
    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(PIPE_WIDTH, PIPE_HEIGHT))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(PIPE_START + PIPE_SEP, PIPE_BASE_Y, 0.),
    ));

    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(PIPE_WIDTH, PIPE_HEIGHT))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(PIPE_START + PIPE_SEP, -PIPE_BASE_Y, 0.),
    ));
}

fn gravity(mut query: Query<(&mut Bird, &mut Transform)>) {
    // Update gravity
    let (mut bird, mut transform) = query.get_single_mut().unwrap();
    bird.velocity += GRAVITY;
    transform.translation.y += bird.velocity;
}

fn flap(mut query: Query<&mut Bird>, keys: Res<ButtonInput<KeyCode>>) {
    // Flap
    let mut bird = query.get_single_mut().unwrap();
    if keys.just_pressed(KeyCode::Space) {
        bird.velocity = BIRD_FLAP;
    }
}

fn move_pipes(mut query: Query<&mut Transform, With<Pipe>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x <= -PIPE_SEP {
            transform.translation.x = PIPE_SEP;
        }

        transform.translation.x -= SCROLL_SPEED;
    }
}

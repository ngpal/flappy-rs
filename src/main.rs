use bevy::prelude::*;

const GRAVITY: f32 = -0.4;

const WINDOW_WIDTH: f32 = 400.;
const WINDOW_HEIGHT: f32 = 500.;

const BIRD_RADIUS: f32 = 20.;
const BIRD_FLAP: f32 = 9.;

const HORIZONTAL_SPEED: f32 = 5.;

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
        Transform::from_xyz(0.0, 100.0, 1.0),
    ));

    // First set
    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(50., 400.))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(600., 275., 0.),
    ));

    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(50., 400.))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(600., -275., 0.),
    ));

    // Second set
    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(50., 400.))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(900., 275., 0.),
    ));

    commands.spawn((
        Pipe::default(),
        Mesh2d(meshes.add(Rectangle::new(50., 400.))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(900., -275., 0.),
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
        if transform.translation.x <= -300. {
            transform.translation.x = 300.;
        }

        transform.translation.x -= HORIZONTAL_SPEED;
    }
}

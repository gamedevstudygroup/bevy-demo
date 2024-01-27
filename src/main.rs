//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_editor_pls::{egui::MouseWheelUnit, prelude::*};
use smart_default::SmartDefault;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, change_speed_system)
        .init_resource::<Speed>()
        .register_type::<Speed>()
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    // TODO: create a set of standard camera controls, all in a folder, such that it is easy to change
    // ???: better to dynamically attach components or include enabled flag in components?
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Pov,
    ));
}

// Define a "Component" Player used to tag the object that should move.
#[derive(Component)]
struct Player;

#[derive(Resource, SmartDefault, Reflect)]
#[reflect(Resource)]
//https://docs.rs/bevy/latest/bevy/reflect/derive.Reflect.html
struct Speed(#[default = 0.5] f32);

#[derive(Component)]
struct Pov;

// //TODO use derivative syntax sugar
// impl Default for Speed {
//     fn default() -> Self {
//         Self(0.5)
//     }
// }

fn change_speed_system(mut speed: ResMut<Speed>, mut input: EventReader<MouseWheel>) {
    for ev in input.read() {
        speed.0 += ev.y / 5f32;
        speed.0 = speed.0.max(0f32);
    }
}

// Lets move the cube with the keyboard
fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    speed: Res<Speed>,
    mut entities: Query<&mut Transform, With<Player>>,
    camera: Query<&Transform, (With<Pov>, Without<Player>)>,
) {
    let up = input.pressed(KeyCode::W);
    let left = input.pressed(KeyCode::A);
    let down = input.pressed(KeyCode::S);
    let right = input.pressed(KeyCode::D);

    let vv = up as i32 - down as i32;
    let vh = right as i32 - left as i32;

    let mut d = Vec3 {
        x: vh as f32,
        y: vv as f32,
        z: 0f32,
    };

    // rotate d by angle of the camera
    d = camera.single().rotation.mul_vec3(d);

    // lock z axis
    d.y = 0f32;

    // lock to unit circle and scale for speed and timestep
    d = d.normalize_or_zero();
    d *= time.delta_seconds() * speed.0;

    for mut e in entities.iter_mut() {
        e.translation += d;
    }
}

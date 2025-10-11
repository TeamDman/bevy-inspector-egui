use std::env;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    let mut args = env::args();
    args.next(); // skip executable name
    let num_extra_entities = args.next()
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap_or(0);
    let num_ticks = args.next()
        .unwrap_or_default()
        .parse::<u32>()
        .inspect(|n| {
            println!("The app will exit after {} ticks (frames).", n);
        })
        .unwrap_or(u32::MAX);
    let start = std::time::Instant::now();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup.with_input(num_extra_entities))
        .add_systems(Update, exit_after_delay.with_input(num_ticks))
        .run();
    println!("App exited after {:?}", start.elapsed());
}
/// set up a simple 3D scene
fn setup(
    InMut(num_extra_entities): InMut<u32>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    // cube
    commands.spawn((
        Name::new("My Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgba(255., 181. / 255., 0., 102. / 255.))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // extra entities
    for i in 0..*num_extra_entities {
        commands.spawn((
            Name::new(format!("Extra Cube {i}")),
            Transform::from_xyz(i as f32 * 2.0, 0.5, 2.0),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 1.0, 0.5))),
        ));
    }
}

fn exit_after_delay(
    InMut(num_ticks): InMut<u32>,
    mut exit: MessageWriter<AppExit>,
) {
    if *num_ticks == 0 {
        exit.write(AppExit::default());
    } else {
        *num_ticks -= 1;
    }
}

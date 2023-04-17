use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Push them down !".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_car)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-10.0, 23., 52.0);
    let radius = translation.length();

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn spawn_car(mut commands: Commands, assets: Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    let car_scene = assets.load("car/car.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: car_scene,
        transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..Default::default()
    });
}

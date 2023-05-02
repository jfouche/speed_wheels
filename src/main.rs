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
        .add_startup_system(spawn_ground)
        .add_startup_system(spawn_car)
        .run();
}

#[derive(Debug)]
enum Wheel {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

struct CarConfig {
    length: f32,
    width: f32,
    height: f32,
    wheel_diameter: f32,
    wheel_width: f32,
}

impl CarConfig {
    const fn default() -> Self {
        CarConfig {
            length: 2.0,
            width: 1.2,
            height: 1.0,
            wheel_diameter: 0.5,
            wheel_width: 0.1,
        }
    }

    fn car_box(&self) -> shape::Box {
        shape::Box::new(self.length, self.height, self.width)
    }

    fn collider(&self) -> Collider {
        Collider::cuboid(self.length / 2., self.height / 2., self.width / 2.)
    }

    fn wheel_shape(&self) -> shape::Cylinder {
        shape::Cylinder {
            height: self.wheel_width,
            radius: self.wheel_diameter / 2.,
            ..default()
        }
    }

    fn wheel_collider(&self) -> Collider {
        Collider::cylinder(self.wheel_width / 2., self.wheel_diameter / 2.)
    }

    fn wheel_pos(&self, wheel: Wheel) -> Vec3 {
        let (x, z) = match wheel {
            Wheel::FrontLeft => (self.length / 2., self.width / 2.),
            Wheel::FrontRight => (self.length / 2., -self.width / 2.),
            Wheel::RearLeft => (-self.length / 2., self.width / 2.),
            Wheel::RearRight => (-self.length / 2., -self.width / 2.),
        };
        Vec3::new(x, self.width / 2., z)
    }
}

const CAR: CarConfig = CarConfig::default();

fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-10.0, 15., 18.0);
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

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Name::new("Ground"),
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(50.0).into()),
                material: materials.add(Color::SEA_GREEN.into()),
                ..default()
            },
        ))
        .insert((RigidBody::Fixed, Collider::cuboid(25., 0.1, 25.)));
}

///
///
fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let design_bundle = (
        Name::new("Car"),
        PbrBundle {
            mesh: meshes.add(CAR.car_box().into()),
            material: materials.add(Color::SILVER.into()),
            transform: Transform::from_xyz(0., 20., 0.),
            ..default()
        },
    );

    let physics_bundle = (RigidBody::Dynamic, CAR.collider());

    commands
        .spawn((design_bundle, physics_bundle))
        .with_children(|car| {
            let wheel_handle = meshes.add(CAR.wheel_shape().into());
            car.spawn(wheel_bundle(Wheel::FrontLeft, &wheel_handle));
            car.spawn(wheel_bundle(Wheel::FrontRight, &wheel_handle));
            car.spawn(wheel_bundle(Wheel::RearLeft, &wheel_handle));
            car.spawn(wheel_bundle(Wheel::RearRight, &wheel_handle));
        });
}

fn wheel_bundle(wheel: Wheel, wheel_handle: &Handle<Mesh>) -> impl Bundle {
    let name = format!("Wheel {wheel:?}");
    let pos = CAR.wheel_pos(wheel);

    let design_bundle = (
        Name::new(name),
        PbrBundle {
            mesh: wheel_handle.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        },
    );

    let physic_bundle = (RigidBody::Dynamic, CAR.wheel_collider());

    (design_bundle, physic_bundle)
}

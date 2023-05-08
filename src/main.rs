use std::f32::consts::PI;

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
        .init_resource::<CarConfig>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_ground)
        .add_startup_system(spawn_car)
        // .add_startup_system(setup_graphics)
        // .add_startup_system(setup_physics)
        .run();
}

#[derive(Debug)]
enum Wheel {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

struct CarWheel<'a> {
    wheel: Wheel,
    car: &'a CarConfig,
}

impl<'a> CarWheel<'a> {
    fn new(wheel: Wheel, car: &'a CarConfig) -> Self {
        CarWheel { wheel, car }
    }

    fn pos(&self) -> Vec3 {
        let (x, z) = match self.wheel {
            Wheel::FrontLeft => (self.car.length / 2., self.car.width / 2.),
            Wheel::FrontRight => (self.car.length / 2., -self.car.width / 2.),
            Wheel::RearLeft => (-self.car.length / 2., self.car.width / 2.),
            Wheel::RearRight => (-self.car.length / 2., -self.car.width / 2.),
        };
        Vec3::new(x, self.car.width / 2., z)
    }
}

#[derive(Resource)]
struct CarConfig {
    length: f32,
    width: f32,
    height: f32,
    wheel_diameter: f32,
    wheel_width: f32,
    // axle_size: f32,
    // axle_length: f32,
}

impl Default for CarConfig {
    fn default() -> Self {
        CarConfig {
            length: 2.0,
            width: 1.2,
            height: 1.0,
            wheel_diameter: 2.0,
            wheel_width: 0.3,
            // axle_size: 0.05,
            // axle_length: 0.1,
        }
    }
}

impl CarConfig {
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

    // fn axle_pos(&self, wheel: Wheel) -> Vec3 {}
}

fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-8., 10., 7.0);
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
            Transform::from,
        ))
        .insert((RigidBody::Fixed, Collider::cuboid(25., 0.1, 25.)));
}

///
///
fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    car: Res<CarConfig>,
) {
    let body_bundle = (
        Name::new("Car"),
        PbrBundle {
            mesh: meshes.add(car.car_box().into()),
            material: materials.add(Color::SILVER.into()),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },
    );

    let physics_bundle = (RigidBody::Dynamic, car.collider());

    commands
        .spawn((body_bundle, physics_bundle))
        .with_children(|parent| {
            // spawn wheels
            let wheel_handle = meshes.add(car.wheel_shape().into());
            spawn_wheel(parent, CarWheel::new(Wheel::FrontLeft, &car), &wheel_handle);
            spawn_wheel(
                parent,
                CarWheel::new(Wheel::FrontRight, &car),
                &wheel_handle,
            );
            spawn_wheel(parent, CarWheel::new(Wheel::RearLeft, &car), &wheel_handle);
            spawn_wheel(parent, CarWheel::new(Wheel::RearRight, &car), &wheel_handle);
        });
}

fn spawn_wheel(parent: &mut ChildBuilder, car_wheel: CarWheel, wheel_handle: &Handle<Mesh>) {
    let name = format!("Wheel {:?}", car_wheel.wheel);
    let pos = car_wheel.pos();
    let joint_builder = RevoluteJointBuilder::new(Vec3::X).local_anchor2(pos);
    let mut transform = Transform::from_translation(pos);
    transform.rotate_axis(Vec3::X, PI / 2.0);

    let body_bundle = (
        Name::new(name),
        PbrBundle {
            mesh: wheel_handle.clone(),
            transform,
            ..default()
        },
    );

    let physic_bundle = (
        RigidBody::Dynamic,
        car_wheel.car.wheel_collider(),
        ImpulseJoint::new(parent.parent_entity(), joint_builder),
    );
    parent.spawn((body_bundle, physic_bundle));
}

// fn axle_bundle(car: &CarConfig, wheel: Wheel, wheel_handle: &Handle<Mesh>) -> impl Bundle {
//     let name = format!("Axle {wheel:?}");
//     let pos = car.axle_pos(wheel);

//     let body_bundle = (
//         Name::new(name),
//         PbrBundle {
//             mesh: wheel_handle.clone(),
//             transform: Transform::from_translation(pos),
//             ..default()
//         },
//     );

//     let physic_bundle = (RigidBody::Dynamic, car.wheel_collider());

//     (body_bundle, physic_bundle)
// }

/// ============================================================
///
///

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 42.0)
            .looking_at(Vec3::new(13.0, 1.0, 1.0), Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    // create_prismatic_joints(&mut commands, Vec3::new(20.0, 10.0, 0.0), 5);
    create_revolute_joints(&mut commands, Vec3::new(20.0, 0.0, 0.0), 3);
    // create_fixed_joints(&mut commands, Vec3::new(0.0, 10.0, 0.0), 5);
    // create_ball_joints(&mut commands, 15);
}

fn create_revolute_joints(commands: &mut Commands, origin: Vec3, num: usize) {
    let rad = 0.4;
    let shift = 2.0;

    let mut curr_parent = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(origin.x, origin.y, 0.0)),
            RigidBody::Fixed,
            Collider::cuboid(rad, rad, rad),
        ))
        .id();

    for i in 0..num {
        // Create four bodies.
        let z = origin.z + i as f32 * shift * 2.0 + shift;
        let positions = [
            Vec3::new(origin.x, origin.y, z),
            Vec3::new(origin.x + shift, origin.y, z),
            Vec3::new(origin.x + shift, origin.y, z + shift),
            Vec3::new(origin.x, origin.y, z + shift),
        ];

        let mut handles = [curr_parent; 4];
        for k in 0..4 {
            handles[k] = commands
                .spawn((
                    TransformBundle::from(Transform::from_translation(positions[k])),
                    RigidBody::Dynamic,
                    Collider::cuboid(rad, rad, rad),
                ))
                .id();
        }

        // Setup four joints.
        let x = Vec3::X;
        let z = Vec3::Z;

        let revs = [
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-shift, 0.0, 0.0)),
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(shift, 0.0, 0.0)),
        ];

        commands
            .entity(handles[0])
            .insert(ImpulseJoint::new(curr_parent, revs[0]));
        commands
            .entity(handles[1])
            .insert(ImpulseJoint::new(handles[0], revs[1]));
        commands
            .entity(handles[2])
            .insert(ImpulseJoint::new(handles[1], revs[2]));
        commands
            .entity(handles[3])
            .insert(ImpulseJoint::new(handles[2], revs[3]));

        curr_parent = handles[3];
    }
}

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::prelude::*;
pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(FlyCameraPlugin)
			.add_startup_system(setup)
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.add_plugin(RapierDebugRenderPlugin::default())
			.add_startup_system(setup_physics)
			.add_system(print_ball_altitude);
	}
}

const CAMERA_POS: Vec3 = Vec3::new(0., 2., 10.);
#[derive(Component)]
struct MainCamera;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// cam
	commands
		.spawn(Camera3dBundle {
			transform: Transform::from_translation(CAMERA_POS),
			..default()
		})
		.insert(MainCamera)
		.insert(FlyCamera::default())
		.insert(Name::from("Main camera"));

	// light
	commands
		.spawn(PointLightBundle {
			point_light: PointLight {
				intensity: 50000.0,
				range: 250.,
				// shadows_enabled: true,
				..default()
			},
			transform: Transform::from_translation(CAMERA_POS + Vec3::Y * 3.),
			..default()
		})
		.insert(Name::new("Main light"));

	// ground plane
	commands
		.spawn((
			PbrBundle {
				mesh: meshes.add(shape::Plane::from_size(500.0).into()),
				material: materials.add(Color::SILVER.into()),
				// transform to be behind, xy plane
				transform: Transform::from_xyz(0., -2., 0.),
				..default()
			},
			// PickableBundle::default(),    // Makes the entity pickable
			// RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
			// OnPointer::<Click>::run_callback(handle_plane_clicked),
		))
		.insert(Collider::cuboid(100.0, 0.1, 100.0))
		.insert(Name::from("Ground"));
}

fn setup_physics(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// /* Create the ground. */
	// commands

	// 	.insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

	/* Create the bouncing ball. */
	commands
		.spawn(RigidBody::Dynamic)
		.insert(Collider::ball(0.5))
		.insert(Restitution::coefficient(0.7))
		.insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
		.insert(PbrBundle {
			mesh: meshes.add(
				bevy::prelude::shape::Icosphere {
					radius: 0.5,
					subdivisions: 4,
				}
				.try_into()
				.unwrap(),
			),
			material: materials.add(Color::RED.into()),
			..default()
		});
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
	for transform in positions.iter() {
		println!("Ball altitude: {}", transform.translation.y);
	}
}

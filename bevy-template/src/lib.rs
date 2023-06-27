use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_mod_picking::{backends::rapier::RapierPickTarget, prelude::*};
use bevy_rapier3d::prelude::*;
use rand::{random, distributions::Standard};

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(setup)
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			// .add_plugin(RapierDebugRenderPlugin::default())
			.add_startup_system(setup_physics)
			.add_system(print_ball_altitude);
	}
}

trait EntityCommandsExt {
	fn with_picking(&mut self) -> &mut Self;

	fn as_pick_camera(&mut self) -> &mut Self;

	fn name(&mut self, name: &'static str) -> &mut Self;
}

impl EntityCommandsExt for EntityCommands<'_, '_, '_> {
	/// Adds bevy_mod_picking magic
	fn with_picking(&mut self) -> &mut Self {
		self
			.insert(PickableBundle::default())
			.insert(RapierPickTarget)
			.insert(RaycastPickTarget::default())
	}

	fn as_pick_camera(&mut self) -> &mut Self {
		self
			.insert(RapierPickCamera::default())
			.insert(RaycastPickCamera::default())
	}

	fn name(&mut self, name: &'static str) -> &mut Self {
		self.insert(Name::new(name))
	}
}

const CAMERA_POS: Vec3 = Vec3::new(0., 5., 15.);
const CAMERA_LOOKING_AT: Vec3 = Vec3::new(0., 5., 0.);

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
			transform: Transform::from_translation(CAMERA_POS).looking_at(CAMERA_LOOKING_AT, Vec3::Y),
			..default()
		})
		.insert(MainCamera)
		.insert(FlyCamera::default())
		.as_pick_camera()
		.name("Main camera");

	// light
	commands
		.spawn(PointLightBundle {
			point_light: PointLight {
				intensity: 500.0,
				range: 2500.,
				shadows_enabled: true,
				..default()
			},
			transform: Transform::from_translation(Vec3::default() + Vec3::Y * 10.),
			..default()
		})
		.name("Main light");

	// ground plane
	const DIF: f32 = 0.1;
	commands
		.spawn((
			PbrBundle {
				mesh: meshes.add(shape::Plane::from_size(500.0).into()),
				material: materials.add(Color::SILVER.into()),
				// transform to be behind, xy plane
				transform: Transform::from_xyz(0., -DIF, 0.),
				..default()
			},
			// PickableBundle::default(),    // Makes the entity pickable
			// RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
			// OnPointer::<Click>::run_callback(handle_plane_clicked),
		))
		.insert(Collider::cuboid(100.0, DIF, 100.0))
		.name("Ground");
}

fn setup_physics(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for i in 0..=100 {
		let mut mat: StandardMaterial = Color::RED.into();
		// mat.emissive = Color::Rgba { red: 1., green: 0., blue: 0., alpha: 0.7 };
		mat.metallic = 1.;
		mat.perceptual_roughness = 1.;
		mat.reflectance = 1.;

		/* Create the bouncing ball. */
		commands
			.spawn(RigidBody::Dynamic)
			.insert(Collider::ball(0.5))
			.insert(PbrBundle {
				mesh: meshes.add(
					bevy::prelude::shape::Icosphere {
						radius: 0.5,
						subdivisions: 16,
					}
					.try_into()
					.unwrap(),
					// shape::Box::new(0.1, 0.1, 0.1).into(),
				),
				material: materials.add(mat),
				transform: Transform::from_xyz(
					0.0 + random::<f32>(),
					5.0 + i as f32,
					0.0 + random::<f32>(),
				),
				..default()
			})
			.insert(Restitution::coefficient(0.7))
			.with_picking()
			.insert(OnPointer::<Over>::run_callback(
				|In(e): In<ListenedEvent<_>>,
				 mut commands: Commands,
				 mut q: Query<&mut Handle<StandardMaterial>>,
				 mut materials: ResMut<Assets<StandardMaterial>>| {
					// commands.entity(e.target).insert(ExternalImpulse {
					// 	impulse: Vec3::new(0., 5., 0.),
					// 	..default()
					// });

					if let Ok(mut handle) = q.get_mut(e.target) {
						let mut mat: StandardMaterial = Color::GREEN.into();
						mat.metallic = 1.;

						*handle = materials.add(mat);
					}

					Bubble::Up
				},
			));
	}

	// spawn walls
	const HEIGHT: f32 = 10.;
	const WIDTH: f32 = 5.;
	const DIF: f32 = 4.;
	const WALL_WIDTH: f32 = 0.1;

	// back wall
	let wall_col = materials.add(Color::BLUE.into());
	commands
		.spawn(RigidBody::Fixed)
		.insert(Collider::cuboid(WIDTH, HEIGHT, WALL_WIDTH))
		.insert(PbrBundle {
			transform: Transform::from_xyz(0., HEIGHT / 2., -DIF / 2.),
			mesh: meshes.add(shape::Box::new(WIDTH, HEIGHT, WALL_WIDTH).into()),
			material: wall_col.clone(),
			..default()
		});

	// front wall
	commands
		.spawn(RigidBody::Fixed)
		.insert(Collider::cuboid(WIDTH, HEIGHT, WALL_WIDTH))
		.insert(PbrBundle {
			transform: Transform::from_xyz(0., HEIGHT / 2., DIF / 2.),
			mesh: meshes.add(shape::Box::new(WIDTH, HEIGHT, WALL_WIDTH).into()),
			material: wall_col.clone(),
			visibility: Visibility::Hidden,
			..default()
		});

	// left wall
	commands
		.spawn(RigidBody::Fixed)
		.insert(Collider::cuboid(WALL_WIDTH, HEIGHT, DIF))
		.insert(PbrBundle {
			transform: Transform::from_xyz(-WIDTH / 2., HEIGHT / 2., 0.),
			mesh: meshes.add(shape::Box::new(WALL_WIDTH, HEIGHT, DIF).into()),
			material: wall_col.clone(),
			visibility: Visibility::Visible,
			..default()
		});

	commands
		.spawn(RigidBody::Fixed)
		.insert(Collider::cuboid(WALL_WIDTH, HEIGHT, DIF))
		.insert(PbrBundle {
			transform: Transform::from_xyz(WIDTH / 2., HEIGHT / 2., 0.),
			mesh: meshes.add(shape::Box::new(WALL_WIDTH, HEIGHT, DIF).into()),
			material: wall_col,
			visibility: Visibility::Visible,
			..default()
		});
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
	for transform in positions.iter() {
		// println!("Ball altitude: {}", transform.translation.y);
	}
}

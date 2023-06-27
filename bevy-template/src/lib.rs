use bevy::prelude::*;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

const CAMERA_HEIGHT: f32 = 10.;
const LIGHT_HEIGHT: f32 = 10.;

#[derive(Component)]
struct MainCamera;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// cam
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0., CAMERA_HEIGHT, 0.),
			..default()
		},
		// RaycastPickCamera::default(),
		MainCamera,
	));

	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 50000.0,
			range: 250.,
			// shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(0., LIGHT_HEIGHT, 0.),
		..default()
	});

	// ground plane
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(shape::Plane::from_size(500.0).into()),
			material: materials.add(Color::SILVER.into()),
			// transform to be behind, xy plane
			transform: Transform::from_xyz(0., 0., 0.),
			..default()
		},
		// PickableBundle::default(),    // Makes the entity pickable
		// RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
		// OnPointer::<Click>::run_callback(handle_plane_clicked),
	));
}

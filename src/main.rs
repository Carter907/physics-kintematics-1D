use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct PhysicsEntity;

#[derive(Component)]
struct Velocity(f64);

#[derive(Component)]
struct Displacement(f64);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsFreeFall))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((PhysicsEntity, Velocity(0f64), Displacement(40f64)));

    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}

#[derive(Resource)]
struct PhysicsTimer;

fn update_entities(
    time: Res<Time>,
    mut timer: ResMut<PhysicsTimer>,
    mut query: Query<(&mut Velocity, &mut Displacement), With<PhysicsEntity>>,
) {
}

struct PhysicsFreeFall;
impl Plugin for PhysicsFreeFall {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

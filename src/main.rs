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
struct PhysicsTimer(Timer);

fn update_entities(
    time: Res<Time>,
    mut timer: ResMut<PhysicsTimer>,
    mut query: Query<(&mut Velocity, &mut Displacement), With<PhysicsEntity>>,
) {
    query.for_each_mut(|mut properties| {
        properties.0 .0 += -9.81 * time.delta().as_secs_f64();
        properties.1 .0 += properties.0 .0 * time.delta().as_secs_f64();
    });
    if timer.0.tick(time.delta()).just_finished() {
        for (velocity, displacement) in &mut query {
            println!("{} {}", velocity.0, displacement.0);
        }
    }
}

struct PhysicsFreeFall;
impl Plugin for PhysicsFreeFall {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysicsTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(Update, update_entities);
    }
}

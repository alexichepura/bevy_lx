use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn ground_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_ground(&mut commands, &mut meshes, &mut materials);
}

pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let multiplier: usize = 2;
    let scale = 280. / multiplier as f32;
    let num_cols: usize = 2 * multiplier;
    let num_rows: usize = 3 * multiplier;
    let hx = num_cols as f32 * scale;
    let hy = 0.5;
    let hz = num_rows as f32 * scale;
    let ground_size: Vec3 = 2. * Vec3::new(hx, hy, hz);
    let heights: Vec<Real> = vec![hy; num_rows * num_cols];
    commands
        .spawn_empty()
        .insert(Name::new("road-heightfield"))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: hx,
                min_x: -hx,
                max_y: hy,
                min_y: -hy,
                max_z: hz,
                min_z: -hz,
            })),
            // material: materials.add(Color::rgb(0.2, 0.35, 0.2).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("7b824e").unwrap(),
                perceptual_roughness: 0.3,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            -350., -hy, 570.,
        )))
        .insert(Collider::heightfield(
            heights,
            num_rows,
            num_cols,
            ground_size.into(),
        ))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(Friction::coefficient(1.))
        .insert(Restitution::coefficient(0.));
}

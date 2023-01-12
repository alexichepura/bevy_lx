use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn container_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let c = LxContainer::fcl();
    for ix in 0..20 {
        for iy in 0..20 {
            for iz in 0..20 {
                let y = c.hh() + iy as f32 * c.h;
                spawn_container(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    Vec3::new(ix as f32 * 3., y, iz as f32 * 13.),
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LxContainer {
    pub w: f32,
    pub h: f32,
    pub l: f32,
}
impl LxContainer {
    pub fn fcl() -> Self {
        LxContainer {
            w: 2.4,
            h: 2.6,
            l: 12.2,
        }
    }
    pub fn hw(&self) -> f32 {
        self.w / 2.
    }
    pub fn hh(&self) -> f32 {
        self.h / 2.
    }
    pub fn hl(&self) -> f32 {
        self.l / 2.
    }
    pub fn shape(&self) -> shape::Box {
        shape::Box::new(self.w, self.h, self.l)
    }
    pub fn collider(&self) -> Collider {
        Collider::cuboid(self.hw(), self.hh(), self.hl())
    }
}

pub fn spawn_container(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tr: Vec3,
) {
    let c = LxContainer::fcl();
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(c.shape().into()),
            material: materials.add(Color::rgb(0.4, 0.2, 0.2).into()),
            transform: Transform::from_translation(tr),
            ..Default::default()
        })
        // .insert(RigidBody::Dynamic)
        .insert(RigidBody::Fixed)
        .insert(Sleeping {
            sleeping: true,
            ..default()
        })
        .insert(Velocity::zero())
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(Restitution::coefficient(0.))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(c.collider());
}

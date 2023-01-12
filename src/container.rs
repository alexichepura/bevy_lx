use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn container_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fcl_gl: Handle<Scene> = asset_server.load("fcl_container.glb#Scene0");

    let c = LxContainer::fcl();
    for ix in 0..20 {
        for iy in 0..20 {
            for iz in 0..20 {
                let y = c.hh() + iy as f32 * c.h;
                let translate = Vec3::new(ix as f32 * 3., y, iz as f32 * 13.);
                let transform = Transform::from_translation(translate);
                let transform = transform.with_rotation(Quat::from_rotation_y(FRAC_PI_2));
                spawn_container(&mut commands, transform, &fcl_gl);
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
    // pub fn shape(&self) -> shape::Box {
    //     shape::Box::new(self.w, self.h, self.l)
    // }
    pub fn collider(&self) -> Collider {
        Collider::cuboid(self.hw(), self.hh(), self.hl())
    }
}

pub fn spawn_container(commands: &mut Commands, transform: Transform, fcl_gl: &Handle<Scene>) {
    let c = LxContainer::fcl();
    commands
        .spawn_empty()
        .insert(TransformBundle::from(transform))
        .insert(SceneBundle {
            scene: fcl_gl.clone(),
            transform,
            ..default()
        })
        .with_children(|children| {
            let transform = Transform::from_translation(Vec3::Y * c.hh());
            children
                .spawn_empty()
                .insert(c.collider())
                .insert(TransformBundle::from(transform))
                .insert(RigidBody::Fixed)
                // .insert(RigidBody::Dynamic)
                .insert(Sleeping {
                    sleeping: true,
                    ..default()
                })
                .insert(Velocity::zero())
                .insert(ColliderScale::Absolute(Vec3::ONE))
                .insert(Restitution::coefficient(0.))
                .insert(ExternalForce::default())
                .insert(ExternalImpulse::default());
        });
}

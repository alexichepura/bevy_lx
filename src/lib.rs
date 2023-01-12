mod camera;
mod container;
mod dash;
mod font;
mod ground;
mod input;
mod light;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, pbr::DirectionalLightShadowMap, prelude::*};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_rapier3d::prelude::*;
use camera::*;
use container::*;
use dash::*;
use font::FontHandle;
use ground::*;
use input::*;
use light::*;

fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
    c.integration_parameters.max_velocity_iterations = 64;
    c.integration_parameters.max_velocity_friction_iterations = 16;
    c.integration_parameters.max_stabilization_iterations = 64;
    c.integration_parameters.allowed_linear_error = 0.0001;
    c.integration_parameters.erp = 0.99;
    dbg!(c.integration_parameters);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemLabel {
    Input,
    Brain,
    Esp,
}

const FPS: f32 = 60.;
pub fn lx_app(app: &mut App) -> &mut App {
    app.add_plugin(FramepacePlugin)
        .init_resource::<FontHandle>()
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 1. / FPS,
                substeps: 20,
            },
            ..default()
        })
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(FPS as f64),
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(CameraConfig::default())
        .insert_resource(DirectionalLightShadowMap { size: 2048 * 4 })
        .add_startup_system(ground_start_system)
        .add_startup_system(container_start_system)
        .add_startup_system(camera_start_system)
        .add_startup_system(light_start_system)
        .add_startup_system(dash_fps_start_system)
        .add_startup_system(rapier_config_start_system)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(camera_controller_system)
        .add_system(camera_switch_system)
        .add_system(keyboard_input_system)
        .add_system(dash_fps_system);

    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
    {
        use bevy_prototype_debug_lines::DebugLinesPlugin;
        app.add_plugin(DebugLinesPlugin::with_depth_test(true))
            .add_plugin(RapierDebugRenderPlugin {
                enabled: false,
                style: DebugRenderStyle {
                    rigid_body_axes_length: 0.5,
                    // subdivisions: 50,
                    ..default()
                },
                // | DebugRenderMode::COLLIDER_AABBS
                mode: DebugRenderMode::COLLIDER_SHAPES
                    | DebugRenderMode::RIGID_BODY_AXES
                    | DebugRenderMode::JOINTS
                    | DebugRenderMode::CONTACTS
                    | DebugRenderMode::SOLVER_CONTACTS,
                ..default()
            });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use bevy_atmosphere::prelude::*;
        app.insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(0.0, 1.0, 1.0),
            ..default()
        }))
        .add_plugin(AtmospherePlugin);
    }

    app
}

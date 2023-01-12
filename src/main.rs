use bevy::prelude::*;
use bevy_lx::lx_app;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "bavy_lx".to_string(),
            width: 720.,
            height: 640.,
            // monitor: MonitorSelection::Index(1),
            position: WindowPosition::Centered,
            fit_canvas_to_parent: true,
            // canvas: Some("#bevy".to_string()),
            ..default()
        },
        ..default()
    }));

    lx_app(&mut app).run();
}

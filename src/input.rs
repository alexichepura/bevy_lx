use bevy::prelude::*;

pub fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))] mut debug_ctx: ResMut<
        bevy_rapier3d::render::DebugRenderContext,
    >,
) {
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
    }
}

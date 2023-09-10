//This file is for testing the camera position and scene. As the camera has to have something to render if clear color does not suffice.

#[cfg(target_os = "windows")]
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    pbr::PbrPlugin,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};
#[cfg(target_os = "windows")]
use wgpu::{Backends, Limits};

#[cfg(target_os = "windows")]
mod test_systems;

#[cfg(target_os = "windows")]
fn main() {
    //Logging should be set up before anything else.

    info!("Hello Rust Bevy Magic Leap One!");

    let mut app = App::new();

    //Plugins for a test app. This might change once rendering works.
    app.add_plugins(
        DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                power_preference: wgpu::PowerPreference::LowPower,
                priority: bevy::render::settings::WgpuSettingsPriority::Functionality,
                limits: Limits::downlevel_defaults(),
                constrained_limits: Some(Limits::downlevel_defaults()),
                ..default()
            },
        }), // .set(PbrPlugin {
            //     prepass_enabled: false,
            // }),
    );

    //app.add_plugins(bevy::app::ScheduleRunnerPlugin::default());

    //Add the temporary cameras that should be added automatically to a XrSkeleton later.
    app.add_systems(Startup, cameras_setup);

    //A test scene for testing.
    use test_systems::*;
    app.add_systems(Update, test)
        .add_systems(Update, gizmos)
        .add_systems(Startup, scene_setup)
        .insert_resource(ClearColor(Color::GOLD));

    app.run();
}

#[cfg(target_os = "windows")]
fn cameras_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        camera: Camera { ..default() },
        transform: Transform::from_xyz(-4.0, 2.0, -4.0).looking_at(Vec3::X + Vec3::Y, Vec3::Y),
        ..default()
    });
}

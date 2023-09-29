use bevy::app::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::prelude::*;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use magicleap_one_lumin_sdk_sys::magicleap_c_api;
use std::os::raw::c_int;
use std::ptr;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use wgpu::Backends;
use wgpu::Limits;

use crate::callbacks;
use crate::graphics;
use crate::log;
use crate::test_systems;

// mod callbacks;
// mod graphics;
// mod log;
// mod test_systems;

pub fn main() {
    //Logging should be set up before anything else.
    log::setup_magicleap_one_tracing();
    info!("Hello Rust Bevy Magic Leap One!");

    let mut app = App::new();

    //Plugins for a test app. This might change once rendering works.
    app.add_plugins(
        DefaultPlugins
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    backends: Some(Backends::GL),
                    // power_preference: wgpu::PowerPreference::LowPower,
                    priority: bevy::render::settings::WgpuSettingsPriority::Functionality,
                    limits: Limits::downlevel_defaults(),
                    constrained_limits: Some(Limits::downlevel_defaults()),
                    ..default()
                },
            })
            // .set(bevy::window::WindowPlugin {
            //     primary_window: None,
            //     exit_condition: bevy::window::ExitCondition::DontExit,
            //     close_when_requested: false,
            // })
            // .set(PbrPlugin {
            // prepass_enabled: false,
            // })
            .disable::<LogPlugin>(),
    )
    .add_plugins(MagicLeapOnePlugin);

    //Add the temporary cameras that should be added automatically to a XrSkeleton later.
    app.add_systems(Startup, graphics::setup_magic_leap_one_cameras);

    //A test scene for testing.
    use test_systems::*;
    app.add_systems(Update, test)
        .add_systems(Update, gizmos)
        ///////////////////////////////////////////////////////////////////////////////////////////////////////
        // Disable below System and the scene renders gizmos!
        ///////////////////////////////////////////////////////////////////////////////////////////////////////
        //.add_systems(Startup, scene_setup)
        .insert_resource(ClearColor(Color::GOLD));

    app.run();
}

#[derive(Default)]
struct MagicLeapOnePlugin;

impl Plugin for MagicLeapOnePlugin {
    fn build(&self, app: &mut App) {
        info!("Hello Bevy Magic Leap One Plugin!");
        app.set_runner(magicleap_one_runner);

        initialize_magicleap_one_start();
    }

    fn finish(&self, app: &mut App) {
        info!("Finish Bevy Magic Leap One Plugin!");
        graphics::create_magicleap_one_graphics_client(app);
        //graphics::setup_magic_leap_one_render_targets(app);
        graphics::initialize_magicleap_one_graphics_frame_render_systems(app);
    }

    fn cleanup(&self, _app: &mut App) {
        info!("Cleanup Bevy Magic Leap One Plugin!");
        initialize_magicleap_one_end();
    }
}

pub static mut APP_STATE: Mutex<MagicLeapOneAppState> = Mutex::new(MagicLeapOneAppState::Running);

#[derive(Resource, Debug, Clone)]
pub enum MagicLeapOneAppState {
    Stopped,
    Paused,
    Running,
    LostFocus,
    Standby,
}

fn initialize_magicleap_one_start() {
    //initialize application lifecycle
    let mut callbacks = magicleap_c_api::MLLifecycleCallbacksEx::default();
    unsafe {
        magicleap_c_api::MLLifecycleCallbacksExInitWrapped(&mut callbacks);
    }

    use callbacks::*;
    callbacks.on_stop = Some(on_stopped);
    callbacks.on_pause = Some(on_resumed);
    callbacks.on_resume = Some(on_resumed);
    callbacks.on_device_active = Some(on_device_active);
    callbacks.on_device_reality = Some(on_device_reality);
    callbacks.on_device_standby = Some(on_device_standby);
    callbacks.on_focus_gained = Some(on_focus_gained);
    callbacks.on_focus_lost = Some(on_focus_lost);
    callbacks.on_new_initarg = Some(on_new_initarg);
    callbacks.on_unload_resources = Some(on_unload_resources);

    unsafe {
        if let Err(result) =
            magicleap_c_api::MLLifecycleInitEx(&mut callbacks, ptr::null_mut()).ok()
        {
            panic!("Couldn't initiate Lifecycle! {:?}", String::from(result))
        }
    }

    //initialize perception system
    let mut settings = magicleap_c_api::MLPerceptionSettings::default();
    unsafe {
        if let Err(result) = magicleap_c_api::MLPerceptionInitSettings(&mut settings).ok() {
            panic!(
                "Couldn't initialize Perception Settings! {:?}",
                String::from(result)
            )
        }
        info!("{:?}", settings);
        if let Err(result) = magicleap_c_api::MLPerceptionStartup(&mut settings).ok() {
            panic!("Couldn't startup Perception! {:?}", String::from(result))
        }
    }
}

fn initialize_magicleap_one_end() {
    unsafe {
        //Signal magic leap one that the app is ready. (This could be in the runner.)
        if let Err(result) = magicleap_c_api::MLLifecycleSetReadyIndication().ok() {
            panic!(
                "MLLifecycleSetReadyIndication failed! {:?}",
                String::from(result)
            )
        }

        //I added this in search of the reason of FrameBegin Timeouts.
        let mut handle: u64 = 0;

        if let Err(result) = magicleap_c_api::MLHeadTrackingCreate(&mut handle).ok() {
            panic!("Create head tracking failed! {:?}", String::from(result))
        }

        let mut static_data = magicleap_c_api::MLHeadTrackingStaticData::default();

        if let Err(result) =
            magicleap_c_api::MLHeadTrackingGetStaticData(handle, &mut static_data).ok()
        {
            panic!(
                "Get static head tracking data failed! {:?}",
                String::from(result)
            )
        }
    }
}

fn magicleap_one_runner(mut app: App) {
    while !app.ready() {
        #[cfg(not(target_arch = "wasm32"))]
        bevy::tasks::tick_global_task_pools_on_main_thread();
    }
    app.finish();
    app.cleanup();

    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

    let start_time = Instant::now();

    loop {
        if let Some(app_exit_events) = app.world.get_resource_mut::<Events<AppExit>>() {
            if let Some(_) = app_exit_event_reader.iter(&app_exit_events).last() {
                break;
            }
        }

        {
            let state = unsafe { APP_STATE.lock().unwrap().clone() };
            match state {
                MagicLeapOneAppState::Stopped => {
                    if let Some(mut app_exit_events) =
                        app.world.get_resource_mut::<Events<AppExit>>()
                    {
                        app_exit_events.send_default();
                    }
                    break;
                }
                MagicLeapOneAppState::Paused => continue,
                MagicLeapOneAppState::Running => {}
                MagicLeapOneAppState::LostFocus => continue,
                MagicLeapOneAppState::Standby => continue,
            }
        }

        app.update();
    }

    let end_time = Instant::now();

    info!("Bevy was running from {start_time:?} to {end_time:?}!");
}

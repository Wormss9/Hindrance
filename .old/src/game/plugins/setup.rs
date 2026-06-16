use crate::game::{components::*, resources::*};
use bevy::{
    camera::ScalingMode,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    ecs::system::NonSendMarker,
    post_process::bloom::Bloom,
    prelude::*,
    winit::WINIT_WINDOWS,
};
use winit::window::Icon;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Fonts>()
            .add_systems(Startup, set_window_icon)
            .add_systems(Startup, add_camera)
            .add_systems(Update, update_countdown);
    }
}

fn set_window_icon(_marker: NonSendMarker) {
    WINIT_WINDOWS.with_borrow(|winit_windows| {
        if winit_windows.windows.len() == 0 {
            return;
        }

        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("assets/icon.png")
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

        for window in winit_windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }

        info!("Window icon set");
    });
}

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 720.,
            },
            ..OrthographicProjection::default_2d()
        }),
        Tonemapping::None,
        Bloom {
            intensity: 0.25,
            ..Default::default()
        },
        DebandDither::Enabled,
    ));
}

fn update_countdown(counters: Query<(&mut Countdown, &mut Text2d)>, time: Res<Time>) {
    for (mut countdown, mut text) in counters {
        countdown.timer.tick(time.delta());
        let time = countdown.timer.remaining_secs();
        let decimals = ((time.fract() * 100.0).trunc()) as u32;
        let time_text = if time > 60. {
            format!("{:.0}:{:02.0}:{:02.0}", time / 60., time % 60., decimals)
        } else {
            format!("{:.0}:{:02.0}", time, decimals)
        };
        text.0 = time_text;
    }
}

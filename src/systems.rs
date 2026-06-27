use bevy::{
    camera::ScalingMode,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    ecs::system::NonSendMarker,
    post_process::bloom::Bloom,
    prelude::*,
    winit::WINIT_WINDOWS,
};
use winit::window::Icon;

pub fn set_window_icon(_marker: NonSendMarker) {
    WINIT_WINDOWS.with_borrow(|winit_windows| {
        if winit_windows.windows.is_empty() {
            return;
        }

        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("assets/icon.ico")
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

pub fn add_camera(mut commands: Commands) {
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

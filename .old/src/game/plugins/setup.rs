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

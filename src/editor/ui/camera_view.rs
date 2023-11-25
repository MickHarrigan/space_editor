use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::egui::{self};

use crate::{
    prelude::EditorTab,
    prefab::component::CameraPlay,
};

#[derive(Resource)]
pub struct CameraViewTab {
    pub viewport_rect: Option<egui::Rect>,
    pub smoothed_dt: f32,
}

impl Default for CameraViewTab {
    fn default() -> Self {
        Self {
            viewport_rect: None,
            smoothed_dt: 0.0,
        }
    }
}

impl EditorTab for CameraViewTab {
    fn ui(&mut self, ui: &mut bevy_egui::egui::Ui, _commands: &mut Commands, world: &mut World) {
        self.viewport_rect = Some(ui.clip_rect());

        //Draw FPS
        let dt = world.get_resource::<Time>().unwrap().delta_seconds();
        self.smoothed_dt = self.smoothed_dt.mul_add(0.98, dt * 0.02);
        ui.colored_label(
            egui::Color32::WHITE,
            format!("FPS: {:.0}", 1.0 / self.smoothed_dt),
        );
    }

    fn title(&self) -> bevy_egui::egui::WidgetText {
        "Camera view".into()
    }
}

pub fn reset_camera_tab_viewport(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut cameras: Query<&mut Camera, With<CameraPlay>>,
) {
    let Ok(mut cam) = cameras.get_single_mut() else {
        return;
    };

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    cam.viewport = Some(bevy::render::camera::Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(window.width() as u32, window.height() as u32),
        depth: 0.0..1.0,
    });
}

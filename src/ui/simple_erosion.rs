use bevy_egui::egui::{Slider, TextEdit, Ui};

use crate::layer::settings::SimpleErosionSettings;

pub fn simple_erosion(ui: &mut Ui, settings: &mut SimpleErosionSettings) -> bool {
    let mut max_angle = settings.max_angle;
    let mut iterations = settings.iterations.to_string();
    let mut erosion_factor = settings.erosion_factor;

    let iterations_ui = ui.add(TextEdit::singleline(&mut iterations));
    if iterations_ui.changed() {
        settings.iterations = iterations.parse::<usize>().unwrap();
        return true;
    }

    let max_angle_ui = ui.add(Slider::new(&mut max_angle, 0.01..=0.05).text("Max Angle:"));
    if max_angle_ui.drag_stopped() {
        settings.max_angle = max_angle;
        return true;
    }

    let erosion_factor_ui =
        ui.add(Slider::new(&mut erosion_factor, 0.0..=1.0).text("Erosion Factor:"));
    if erosion_factor_ui.drag_stopped() {
        settings.erosion_factor = erosion_factor;
        return true;
    }

    false
}

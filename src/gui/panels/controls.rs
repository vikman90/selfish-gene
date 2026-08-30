//! Control toolbar with playback, speed settings, and tab navigation.

use crate::gui::state::{ActiveTab, RunnerState, SimulationSpeed};
use egui::{Color32, RichText, Ui};

/// Actions triggered by the control bar
pub enum ControlAction {
    Start,
    Pause,
    Step,
    Reset,
}

/// Render top control toolbar
pub fn render_controls_bar(
    ui: &mut Ui,
    runner_state: RunnerState,
    speed: &mut SimulationSpeed,
    active_tab: &mut ActiveTab,
) -> Option<ControlAction> {
    let mut action = None;

    ui.horizontal(|ui| {
        // Simulation buttons
        if runner_state.is_running() {
            if ui
                .button(RichText::new("⏸ Pausar").strong().color(Color32::from_rgb(255, 190, 70)))
                .clicked()
            {
                action = Some(ControlAction::Pause);
            }
        } else {
            let label = if runner_state == RunnerState::Idle {
                "▶ Iniciar"
            } else {
                "▶ Continuar"
            };
            if ui
                .button(RichText::new(label).strong().color(Color32::from_rgb(100, 220, 100)))
                .clicked()
            {
                action = Some(ControlAction::Start);
            }
        }

        let can_step = !runner_state.is_running()
            && runner_state != RunnerState::Converged
            && runner_state != RunnerState::MaxTimestepsReached;

        if ui
            .add_enabled(can_step, egui::Button::new("⏭ Paso (+1)"))
            .clicked()
        {
            action = Some(ControlAction::Step);
        }

        if ui.button("🔄 Reiniciar").clicked() {
            action = Some(ControlAction::Reset);
        }

        ui.separator();

        // Speed control
        ui.label("Velocidad:");
        ui.add(egui::Slider::new(&mut speed.steps_per_frame, 1..=50).text("pasos/frame"));

        ui.separator();

        // Tabs
        ui.selectable_value(active_tab, ActiveTab::TimeSeries, "📈 Series Temporales");
        ui.selectable_value(active_tab, ActiveTab::Profiles, "📊 Perfiles Genéticos");
        ui.selectable_value(active_tab, ActiveTab::Telemetry, "📋 Telemetría");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (bg_color, text_color) = match runner_state {
                RunnerState::Running => (Color32::from_rgb(30, 80, 30), Color32::LIGHT_GREEN),
                RunnerState::Paused => (Color32::from_rgb(80, 70, 20), Color32::KHAKI),
                RunnerState::Converged => (Color32::from_rgb(20, 50, 90), Color32::LIGHT_BLUE),
                RunnerState::MaxTimestepsReached => (Color32::from_rgb(80, 40, 20), Color32::GOLD),
                RunnerState::Idle => (Color32::from_rgb(40, 40, 40), Color32::LIGHT_GRAY),
                RunnerState::Interrupted => (Color32::from_rgb(80, 20, 20), Color32::LIGHT_RED),
            };

            ui.label(
                RichText::new(format!(" {} ", runner_state.display_label()))
                    .background_color(bg_color)
                    .color(text_color)
                    .strong(),
            );
        });
    });

    action
}

//! Telemetry dashboard and JSON export panel.

use crate::simulation::Simulation;
use egui::{Color32, Grid, RichText, Ui};

/// Render telemetry and export panel
pub fn render_telemetry_panel(
    ui: &mut Ui,
    simulation: &Simulation,
    export_filename: &mut String,
    export_status: &mut Option<(String, bool)>,
) {
    ui.heading("📋 Simulation Telemetry");
    ui.separator();

    let stats = simulation.current_stats();
    let config = simulation.config();

    egui::ScrollArea::vertical().show(ui, |ui| {
        // --- Metrics Grid ---
        ui.label(RichText::new("Current Ecosystem State").strong());
        Grid::new("telemetry_grid")
            .striped(true)
            .min_col_width(120.0)
            .show(ui, |ui| {
                ui.label("Generation (Timestep):");
                ui.label(RichText::new(format!("{}", simulation.timestep())).strong());
                ui.end_row();

                ui.label("Active population (N):");
                ui.label(
                    RichText::new(format!("{} / {}", stats.population_size, config.capacity))
                        .strong(),
                );
                ui.end_row();

                ui.label("Resource Factor (1 - N/C):");
                let rf = simulation.resource_factor();
                ui.label(format!("{:.4}", rf));
                ui.end_row();

                ui.label("Mean survival μ(S):");
                ui.label(format!(
                    "{:.4} (σ = {:.4})",
                    stats.survival_mean, stats.survival_std
                ));
                ui.end_row();

                ui.label("Mean replication μ(R):");
                ui.label(format!(
                    "{:.4} (σ = {:.4})",
                    stats.replication_mean, stats.replication_std
                ));
                ui.end_row();

                ui.label("Mean mutation μ(M):");
                ui.label(format!(
                    "{:.4} (σ = {:.4})",
                    stats.mutation_mean, stats.mutation_std
                ));
                ui.end_row();

                ui.label("Mean aggression μ(A):");
                ui.label(format!(
                    "{:.4} (σ = {:.4})",
                    stats.aggression_mean, stats.aggression_std
                ));
                ui.end_row();

                ui.label("Mean age μ(Age):");
                ui.label(format!("{:.2} (σ = {:.2})", stats.age_mean, stats.age_std));
                ui.end_row();

                ui.label("Max trait variance:");
                ui.label(format!(
                    "{:.6} (ESS threshold = {:.6})",
                    stats.max_variance(),
                    config.convergence_threshold
                ));
                ui.end_row();
            });

        ui.add_space(16.0);

        // --- Winner Replicator Card ---
        ui.label(RichText::new("🏆 Representative / Winner Profile").strong());
        if let Some(winner) = simulation.winner_profile() {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    render_trait_badge(
                        ui,
                        "Survival",
                        winner.survival_rate,
                        Color32::from_rgb(80, 180, 250),
                    );
                    render_trait_badge(
                        ui,
                        "Replication",
                        winner.replication_rate,
                        Color32::from_rgb(100, 230, 120),
                    );
                    render_trait_badge(
                        ui,
                        "Mutation",
                        winner.mutation_rate,
                        Color32::from_rgb(255, 160, 60),
                    );
                    render_trait_badge(
                        ui,
                        "Aggression",
                        winner.aggression,
                        Color32::from_rgb(255, 110, 110),
                    );
                    ui.label(format!("Age: {}", winner.age));
                });
            });
        } else {
            ui.label(RichText::new("No individuals in population").italics());
        }

        ui.add_space(20.0);

        // --- Export Section ---
        ui.label(RichText::new("💾 Export Results (JSON)").strong());
        ui.horizontal(|ui| {
            ui.label("File:");
            ui.text_edit_singleline(export_filename);

            if ui.button("📥 Export History").clicked() {
                match simulation.history().export_json(export_filename) {
                    Ok(_) => {
                        *export_status = Some((
                            format!("Successfully exported to '{}'!", export_filename),
                            true,
                        ));
                    }
                    Err(e) => {
                        *export_status = Some((format!("Export error: {}", e), false));
                    }
                }
            }
        });

        if let Some((msg, success)) = export_status {
            let color = if *success {
                Color32::LIGHT_GREEN
            } else {
                Color32::LIGHT_RED
            };
            ui.label(RichText::new(msg.as_str()).color(color));
        }
    });
}

fn render_trait_badge(ui: &mut Ui, name: &str, val: f64, color: Color32) {
    ui.label(
        RichText::new(format!("{}: {:.4}", name, val))
            .color(color)
            .strong(),
    );
    ui.separator();
}

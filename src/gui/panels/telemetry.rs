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
    ui.heading("📋 Telemetría de la Simulación");
    ui.separator();

    let stats = simulation.current_stats();
    let config = simulation.config();

    egui::ScrollArea::vertical().show(ui, |ui| {
        // --- Metrics Grid ---
        ui.label(RichText::new("Estado Actual del Ecosistema").strong());
        Grid::new("telemetry_grid")
            .striped(true)
            .min_col_width(120.0)
            .show(ui, |ui| {
                ui.label("Generación (Timestep):");
                ui.label(RichText::new(format!("{}", simulation.timestep())).strong());
                ui.end_row();

                ui.label("Población activa (N):");
                ui.label(RichText::new(format!("{} / {}", stats.population_size, config.capacity)).strong());
                ui.end_row();

                ui.label("Factor de Recursos (1 - N/C):");
                let rf = simulation.resource_factor();
                ui.label(format!("{:.4}", rf));
                ui.end_row();

                ui.label("Supervivencia media μ(S):");
                ui.label(format!("{:.4} (σ = {:.4})", stats.survival_mean, stats.survival_std));
                ui.end_row();

                ui.label("Replicación media μ(R):");
                ui.label(format!("{:.4} (σ = {:.4})", stats.replication_mean, stats.replication_std));
                ui.end_row();

                ui.label("Mutación media μ(M):");
                ui.label(format!("{:.4} (σ = {:.4})", stats.mutation_mean, stats.mutation_std));
                ui.end_row();

                ui.label("Edad media μ(Age):");
                ui.label(format!("{:.2} (σ = {:.2})", stats.age_mean, stats.age_std));
                ui.end_row();

                ui.label("Varianza máxima de rasgos:");
                ui.label(format!("{:.6} (Umbral ESS = {:.6})", stats.max_variance(), config.convergence_threshold));
                ui.end_row();
            });

        ui.add_space(16.0);

        // --- Winner Replicator Card ---
        ui.label(RichText::new("🏆 Perfil Ganador / Representativo").strong());
        if let Some(winner) = simulation.winner_profile() {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    render_trait_badge(ui, "Supervivencia", winner.survival_rate, Color32::from_rgb(80, 180, 250));
                    render_trait_badge(ui, "Replicación", winner.replication_rate, Color32::from_rgb(100, 230, 120));
                    render_trait_badge(ui, "Mutación", winner.mutation_rate, Color32::from_rgb(255, 160, 60));
                    ui.label(format!("Edad: {}", winner.age));
                });
            });
        } else {
            ui.label(RichText::new("No hay individuos en la población").italics());
        }

        ui.add_space(20.0);

        // --- Export Section ---
        ui.label(RichText::new("💾 Exportación de Resultados (JSON)").strong());
        ui.horizontal(|ui| {
            ui.label("Archivo:");
            ui.text_edit_singleline(export_filename);

            if ui.button("📥 Exportar Historial").clicked() {
                match simulation.history().export_json(export_filename) {
                    Ok(_) => {
                        *export_status = Some((format!("¡Exportado con éxito a '{}'!", export_filename), true));
                    }
                    Err(e) => {
                        *export_status = Some((format!("Error al exportar: {}", e), false));
                    }
                }
            }
        });

        if let Some((msg, success)) = export_status {
            let color = if *success { Color32::LIGHT_GREEN } else { Color32::LIGHT_RED };
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

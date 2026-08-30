//! Time series visualization panel using egui_plot.

use crate::stats::SimulationHistory;
use egui::{Color32, Ui};
use egui_plot::{Legend, Line, Plot, PlotPoints};

/// Visibility toggles for time series plot
#[derive(Debug, Clone)]
pub struct SeriesToggles {
    pub show_survival: bool,
    pub show_replication: bool,
    pub show_mutation: bool,
    pub show_age: bool,
}

impl Default for SeriesToggles {
    fn default() -> Self {
        Self {
            show_survival: true,
            show_replication: true,
            show_mutation: true,
            show_age: false,
        }
    }
}

/// Render the time series plot panel
pub fn render_time_series_panel(
    ui: &mut Ui,
    history: &SimulationHistory,
    capacity: usize,
    toggles: &mut SeriesToggles,
) {
    ui.horizontal(|ui| {
        ui.label("Mostrar rasgos:");
        ui.checkbox(&mut toggles.show_survival, "Supervivencia (S)");
        ui.checkbox(&mut toggles.show_replication, "Replicación (R)");
        ui.checkbox(&mut toggles.show_mutation, "Mutación (M)");
        ui.checkbox(&mut toggles.show_age, "Edad Media (Age)");
    });

    ui.separator();

    let total_height = ui.available_height();
    let top_height = (total_height * 0.45).max(120.0);
    let bottom_height = (total_height * 0.48).max(140.0);

    // --- Top Plot: Population vs Capacity ---
    ui.label(egui::RichText::new("Población Total N(t) vs Capacidad C").strong());
    Plot::new("population_plot")
        .height(top_height)
        .legend(Legend::default())
        .show_axes([true, true])
        .show(ui, |plot_ui| {
            let pop_points: PlotPoints = history
                .stats
                .iter()
                .map(|s| [s.timestep as f64, s.population_size as f64])
                .collect();
            let pop_line = Line::new(pop_points)
                .color(Color32::from_rgb(100, 200, 255))
                .name("Población N(t)")
                .width(2.0_f32);
            plot_ui.line(pop_line);

            if let Some(first) = history.stats.first() {
                let last = history.stats.last().unwrap_or(first);
                let cap_points = vec![
                    [first.timestep as f64, capacity as f64],
                    [last.timestep as f64, capacity as f64],
                ];
                let cap_line = Line::new(cap_points)
                    .color(Color32::from_rgb(240, 100, 100))
                    .name("Capacidad C")
                    .style(egui_plot::LineStyle::Dashed { length: 8.0 });
                plot_ui.line(cap_line);
            }
        });

    ui.add_space(8.0);

    // --- Bottom Plot: Trait Means ---
    ui.label(egui::RichText::new("Evolución de Rasgos Fenotípicos Medios").strong());
    Plot::new("traits_plot")
        .height(bottom_height)
        .legend(Legend::default())
        .show_axes([true, true])
        .show(ui, |plot_ui| {
            if toggles.show_survival {
                let surv_points: PlotPoints = history
                    .stats
                    .iter()
                    .map(|s| [s.timestep as f64, s.survival_mean])
                    .collect();
                plot_ui.line(
                    Line::new(surv_points)
                        .color(Color32::from_rgb(80, 180, 250))
                        .name("Supervivencia μ(S)")
                        .width(2.0_f32),
                );
            }

            if toggles.show_replication {
                let rep_points: PlotPoints = history
                    .stats
                    .iter()
                    .map(|s| [s.timestep as f64, s.replication_mean])
                    .collect();
                plot_ui.line(
                    Line::new(rep_points)
                        .color(Color32::from_rgb(100, 230, 120))
                        .name("Replicación μ(R)")
                        .width(2.0_f32),
                );
            }

            if toggles.show_mutation {
                // Scale mutation by 10x for visibility or plot directly
                let mut_points: PlotPoints = history
                    .stats
                    .iter()
                    .map(|s| [s.timestep as f64, s.mutation_mean])
                    .collect();
                plot_ui.line(
                    Line::new(mut_points)
                        .color(Color32::from_rgb(255, 160, 60))
                        .name("Mutación μ(M)")
                        .width(2.0_f32),
                );
            }

            if toggles.show_age {
                let age_points: PlotPoints = history
                    .stats
                    .iter()
                    .map(|s| [s.timestep as f64, s.age_mean])
                    .collect();
                plot_ui.line(
                    Line::new(age_points)
                        .color(Color32::from_rgb(200, 130, 255))
                        .name("Edad media μ(Age)")
                        .width(1.5_f32),
                );
            }
        });
}

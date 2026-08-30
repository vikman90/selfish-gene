//! Parameters configuration panel.

use crate::config::Config;
use egui::{CollapsingHeader, Slider, Ui};

/// Render the parameter configuration sidebar
pub fn render_params_panel(
    ui: &mut Ui,
    config: &mut Config,
    use_seed: &mut bool,
    seed_input: &mut u64,
    on_reset: &mut bool,
) {
    ui.heading("⚙ Parámetros");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        CollapsingHeader::new("🌐 Capacidad y Entorno")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Capacidad de carga (C):");
                ui.add(Slider::new(&mut config.capacity, 100..=50_000).logarithmic(true))
                    .on_hover_text("Límite máximo de replicadores que el entorno puede sostener.");

                ui.label("Tasa de aparición (λ):");
                ui.add(Slider::new(&mut config.appearance_rate, 0.0..=1000.0))
                    .on_hover_text("Tasa de generación espontánea por distribución de Poisson.");

                ui.label("Senescencia (δ):");
                ui.add(Slider::new(&mut config.senescence_rate, 0.0..=0.2).step_by(0.005))
                    .on_hover_text(
                        "Factor de mortalidad exponencial dependiente de la edad (exp(-δ * edad)).",
                    );
            });

        ui.add_space(8.0);

        CollapsingHeader::new("🧬 Rasgos Iniciales")
            .default_open(true)
            .show(ui, |ui| {
                ui.strong("Supervivencia (S):");
                ui.horizontal(|ui| {
                    ui.label("Media:");
                    ui.add(Slider::new(&mut config.init_survival_mean, 0.0..=1.0).step_by(0.01));
                });
                ui.horizontal(|ui| {
                    ui.label("Desv. Est:");
                    ui.add(Slider::new(&mut config.init_survival_std, 0.0..=0.5).step_by(0.01));
                });

                ui.separator();
                ui.strong("Replicación (R):");
                ui.horizontal(|ui| {
                    ui.label("Media:");
                    ui.add(Slider::new(&mut config.init_replication_mean, 0.0..=5.0).step_by(0.05));
                });
                ui.horizontal(|ui| {
                    ui.label("Desv. Est:");
                    ui.add(Slider::new(&mut config.init_replication_std, 0.0..=1.0).step_by(0.01));
                });

                ui.separator();
                ui.strong("Mutación (M):");
                ui.horizontal(|ui| {
                    ui.label("Media:");
                    ui.add(Slider::new(&mut config.init_mutation_mean, 0.0..=0.2).step_by(0.001));
                });
                ui.horizontal(|ui| {
                    ui.label("Desv. Est:");
                    ui.add(Slider::new(&mut config.init_mutation_std, 0.0..=0.05).step_by(0.001));
                });
            });

        ui.add_space(8.0);

        CollapsingHeader::new("📈 Dinámica Evolutiva")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Sigma de mutación (σ):");
                ui.add(Slider::new(&mut config.mutation_sigma, 0.001..=0.1).step_by(0.001))
                    .on_hover_text(
                        "Desviación estándar del ruido gaussiano añadido a los rasgos al mutar.",
                    );

                ui.label("Límite de pasos (0 = sin límite):");
                ui.add(Slider::new(&mut config.max_timesteps, 0..=10_000))
                    .on_hover_text("Número máximo de generaciones antes de finalizar.");

                ui.horizontal(|ui| {
                    if ui.checkbox(use_seed, "Fijar semilla (Seed)").changed() {
                        config.seed = if *use_seed { Some(*seed_input) } else { None };
                    }
                    if *use_seed && ui.add(egui::DragValue::new(seed_input)).changed() {
                        config.seed = Some(*seed_input);
                    }
                });
            });

        ui.add_space(8.0);

        CollapsingHeader::new("🎯 Convergencia")
            .default_open(false)
            .show(ui, |ui| {
                ui.label("Umbral de varianza:");
                ui.add(
                    Slider::new(&mut config.convergence_threshold, 0.0001..=0.01).logarithmic(true),
                )
                .on_hover_text("Varianza máxima para considerar un rasgo estabilizado.");

                ui.label("Ventana de estabilidad:");
                ui.add(Slider::new(&mut config.convergence_window, 5..=200))
                    .on_hover_text("Número de timesteps consecutivos que deben cumplir el umbral.");

                ui.label("Perfiles principales a mostrar:");
                ui.add(Slider::new(&mut config.top_profiles, 5..=30));
            });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("🔄 Aplicar y Reiniciar").clicked() {
                *on_reset = true;
            }
            if ui.button("Restablecer por defecto").clicked() {
                *config = Config::default();
                *use_seed = false;
                *on_reset = true;
            }
        });
    });
}

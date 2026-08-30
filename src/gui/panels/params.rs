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
    ui.heading("⚙ Parameters");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        CollapsingHeader::new("🌐 Capacity & Environment")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Carrying capacity (C):");
                ui.add(Slider::new(&mut config.capacity, 100..=50_000).logarithmic(true))
                    .on_hover_text("Maximum number of replicators the environment can sustain.");

                ui.label("Appearance rate (λ):");
                ui.add(Slider::new(&mut config.appearance_rate, 0.0..=1000.0))
                    .on_hover_text("Spontaneous arrival rate sampled from a Poisson distribution.");

                ui.label("Senescence (δ):");
                ui.add(Slider::new(&mut config.senescence_rate, 0.0..=0.2).step_by(0.005))
                    .on_hover_text(
                        "Age-dependent mortality factor with exponential decay: exp(-δ * age).",
                    );
            });

        ui.add_space(8.0);

        CollapsingHeader::new("🧬 Initial Traits")
            .default_open(true)
            .show(ui, |ui| {
                ui.strong("Survival (S):");
                ui.horizontal(|ui| {
                    ui.label("Mean:");
                    ui.add(Slider::new(&mut config.init_survival_mean, 0.0..=1.0).step_by(0.01));
                });
                ui.horizontal(|ui| {
                    ui.label("Std Dev:");
                    ui.add(Slider::new(&mut config.init_survival_std, 0.0..=0.5).step_by(0.01));
                });

                ui.separator();
                ui.strong("Replication (R):");
                ui.horizontal(|ui| {
                    ui.label("Mean:");
                    ui.add(Slider::new(&mut config.init_replication_mean, 0.0..=5.0).step_by(0.05));
                });
                ui.horizontal(|ui| {
                    ui.label("Std Dev:");
                    ui.add(Slider::new(&mut config.init_replication_std, 0.0..=1.0).step_by(0.01));
                });

                ui.separator();
                ui.strong("Mutation (M):");
                ui.horizontal(|ui| {
                    ui.label("Mean:");
                    ui.add(Slider::new(&mut config.init_mutation_mean, 0.0..=0.2).step_by(0.001));
                });
                ui.horizontal(|ui| {
                    ui.label("Std Dev:");
                    ui.add(Slider::new(&mut config.init_mutation_std, 0.0..=0.05).step_by(0.001));
                });
            });

        ui.add_space(8.0);

        CollapsingHeader::new("📈 Evolutionary Dynamics")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Mutation sigma (σ):");
                ui.add(Slider::new(&mut config.mutation_sigma, 0.001..=0.1).step_by(0.001))
                    .on_hover_text(
                        "Standard deviation of Gaussian noise added to heritable traits during mutation.",
                    );

                ui.label("Max timesteps (0 = unlimited):");
                ui.add(Slider::new(&mut config.max_timesteps, 0..=10_000))
                    .on_hover_text("Maximum number of generations before stopping.");

                ui.horizontal(|ui| {
                    if ui.checkbox(use_seed, "Fix PRNG Seed").changed() {
                        config.seed = if *use_seed { Some(*seed_input) } else { None };
                    }
                    if *use_seed && ui.add(egui::DragValue::new(seed_input)).changed() {
                        config.seed = Some(*seed_input);
                    }
                });
            });

        ui.add_space(8.0);

        CollapsingHeader::new("🎯 Convergence")
            .default_open(false)
            .show(ui, |ui| {
                ui.label("Variance threshold:");
                ui.add(
                    Slider::new(&mut config.convergence_threshold, 0.0001..=0.01).logarithmic(true),
                )
                .on_hover_text("Maximum trait variance required to declare genetic stability (ESS).");

                ui.label("Stability window:");
                ui.add(Slider::new(&mut config.convergence_window, 5..=200))
                    .on_hover_text("Number of consecutive generations that must stay below variance threshold.");

                ui.label("Top profiles to display:");
                ui.add(Slider::new(&mut config.top_profiles, 5..=30));
            });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("🔄 Apply & Reset").clicked() {
                *on_reset = true;
            }
            if ui.button("Reset to Defaults").clicked() {
                *config = Config::default();
                *use_seed = false;
                *on_reset = true;
            }
        });
    });
}

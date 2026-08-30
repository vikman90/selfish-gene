//! Genotype / phenotype profile distribution panel.

use crate::replicator::Replicator;
use egui::{Color32, ProgressBar, RichText, Ui};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProfileBin {
    survival_bin: u8,    // 0-10 (0.0-1.0 in steps of 0.1)
    replication_bin: u8, // 0-20 (0.0-2.0 in steps of 0.1)
    mutation_bin: u8,    // 0-5 (0.0-0.05 in steps of 0.01)
}

impl ProfileBin {
    fn from_replicator(rep: &Replicator) -> Self {
        Self {
            survival_bin: ((rep.survival_rate * 10.0).round() as u8).min(10),
            replication_bin: ((rep.replication_rate * 10.0).round() as u8).min(20),
            mutation_bin: ((rep.mutation_rate * 100.0).round() as u8).min(5),
        }
    }

    fn display_label(&self) -> String {
        format!(
            "S: {:.1}  |  R: {:.1}  |  M: {:.2}",
            self.survival_bin as f64 / 10.0,
            self.replication_bin as f64 / 10.0,
            self.mutation_bin as f64 / 100.0
        )
    }
}

/// Render the profile distribution histogram panel
pub fn render_profiles_panel(ui: &mut Ui, population: &[Replicator], top_n: usize) {
    ui.heading("📊 Genetic Profile Distribution (Phenotypes)");
    ui.label(
        "Active replicator clusters grouped by Survival (S), Replication (R), and Mutation (M) traits.",
    );
    ui.separator();

    if population.is_empty() {
        ui.label(RichText::new("Empty population (extinction)").color(Color32::LIGHT_RED));
        return;
    }

    // Count profiles
    let mut histogram: HashMap<ProfileBin, usize> = HashMap::new();
    for rep in population {
        let bin = ProfileBin::from_replicator(rep);
        *histogram.entry(bin).or_insert(0) += 1;
    }

    let mut sorted_profiles: Vec<(ProfileBin, usize)> = histogram.into_iter().collect();
    sorted_profiles.sort_by_key(|a| std::cmp::Reverse(a.1));

    let total = population.len() as f64;
    let distinct_count = sorted_profiles.len();

    ui.horizontal(|ui| {
        ui.label(format!("Active individuals: {}", population.len()));
        ui.separator();
        ui.label(format!("Unique genetic profiles: {}", distinct_count));
        ui.separator();
        ui.label(format!("Displaying top {}", top_n.min(distinct_count)));
    });

    ui.add_space(8.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, (bin, count)) in sorted_profiles.iter().take(top_n).enumerate() {
            let pct = (*count as f64) / total;
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("#{:02}", i + 1)).monospace().strong());
                ui.label(RichText::new(bin.display_label()).monospace());

                let bar = ProgressBar::new(pct as f32).show_percentage().text(format!(
                    "{} ind. ({:.1}%)",
                    count,
                    pct * 100.0
                ));
                ui.add_sized([ui.available_width() - 10.0, 20.0], bar);
            });
            ui.add_space(4.0);
        }
    });
}

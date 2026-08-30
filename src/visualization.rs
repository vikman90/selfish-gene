use crate::replicator::Replicator;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::collections::HashMap;
use std::io::{stdout, Write};

/// Bins for grouping similar replicators together
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProfileBin {
    survival_bin: u8,    // 0-10 (0.0-1.0 in steps of 0.1)
    replication_bin: u8, // 0-20 (0.0-2.0 in steps of 0.1)
    mutation_bin: u8,    // 0-5 (0.0-0.05 in steps of 0.01)
    aggression_bin: u8,  // 0-10 (0.0-1.0 in steps of 0.1)
}

impl ProfileBin {
    fn from_replicator(rep: &Replicator) -> Self {
        Self {
            survival_bin: ((rep.survival_rate * 10.0).round() as u8).min(10),
            replication_bin: ((rep.replication_rate * 10.0).round() as u8).min(20),
            mutation_bin: ((rep.mutation_rate * 100.0).round() as u8).min(5),
            aggression_bin: ((rep.aggression * 10.0).round() as u8).min(10),
        }
    }

    fn display_label(&self) -> String {
        format!(
            "S:{:.1} R:{:.1} M:{:.2} A:{:.1}",
            self.survival_bin as f64 / 10.0,
            self.replication_bin as f64 / 10.0,
            self.mutation_bin as f64 / 100.0,
            self.aggression_bin as f64 / 10.0,
        )
    }
}

/// Live terminal visualizer for replicator profiles
pub struct LiveVisualizer {
    max_bar_width: usize,
    top_n: usize,
    enabled: bool,
}

impl LiveVisualizer {
    pub fn new(top_n: usize, enabled: bool) -> Self {
        if enabled {
            let _ = enable_raw_mode();
            let _ = execute!(stdout(), Hide, Clear(ClearType::All));
        }

        // Total width: 78 chars (matching the border)
        // Layout: "║ " (2) + label (24) + " │ " (3) + bar (41) + " " (1) + count (5) + " ║" (2) = 78
        Self {
            max_bar_width: 41,
            top_n,
            enabled,
        }
    }

    /// Create histogram of profiles and display as bar chart
    pub fn display(&self, timestep: usize, population: &[Replicator], capacity: usize) {
        if !self.enabled {
            return;
        }

        // Group replicators into bins
        let mut histogram: HashMap<ProfileBin, usize> = HashMap::new();
        for rep in population {
            let bin = ProfileBin::from_replicator(rep);
            *histogram.entry(bin).or_insert(0) += 1;
        }

        // Sort by count (descending)
        let mut sorted: Vec<_> = histogram.into_iter().collect();
        sorted.sort_by_key(|a| std::cmp::Reverse(a.1));

        let mut stdout = stdout();

        // Build output buffer
        let mut output = String::new();

        // Header
        output.push_str(
            "╔════════════════════════════════════════════════════════════════════════════╗\r\n",
        );
        output.push_str(&format!(
            "║ Timestep: {:6}  |  Population: {:5} / {:5}                             ║\r\n",
            timestep,
            population.len(),
            capacity
        ));
        output.push_str(
            "╠════════════════════════════════════════════════════════════════════════════╣\r\n",
        );

        // Format "Profile Distribution" line with proper width
        let profile_text = format!("Profile Distribution (Top {} profiles)", self.top_n);
        output.push_str(&format!("║ {:<74} ║\r\n", profile_text));
        output.push_str(
            "╠════════════════════════════════════════════════════════════════════════════╣\r\n",
        );

        // Display top N profiles
        let max_count = sorted.first().map(|(_, count)| *count).unwrap_or(1);

        for (bin, count) in sorted.iter().take(self.top_n) {
            let bar_length = if max_count > 0 {
                ((*count as f64 / max_count as f64) * self.max_bar_width as f64) as usize
            } else {
                0
            };

            // Profile label (left-aligned, fixed width)
            output.push_str(&format!("║ {:24} │ ", bin.display_label()));

            // Bar with padding to fill the space
            let bar = "█".repeat(bar_length);
            let padding = " ".repeat(self.max_bar_width.saturating_sub(bar_length));
            output.push_str(&format!("\x1b[32m{}\x1b[0m{}", bar, padding));
            output.push_str(&format!(" {:>5} ║\r\n", count));
        }

        // Fill remaining lines with empty rows if we have fewer than top_n profiles
        for _ in sorted.len()..self.top_n {
            // Empty line matching the format: label (24 chars) + bar area (41 chars) + count (5 chars)
            output.push_str(&format!(
                "║ {:24} │ {} {:>5} ║\r\n",
                "",
                " ".repeat(self.max_bar_width),
                ""
            ));
        }

        // Footer
        output.push_str(
            "╚════════════════════════════════════════════════════════════════════════════╝\r\n",
        );
        output.push_str("\r\n");
        output.push_str("Legend: S=Survival, R=Replication, M=Mutation, A=Aggression (Hawk)\r\n");

        // Clear screen, move to top, and print all at once
        let _ = queue!(stdout, MoveTo(0, 0));
        let _ = write!(stdout, "{}", output);
        let _ = stdout.flush();
    }

    /// Display final statistics (non-refreshing)
    pub fn display_final(&self, population: &[Replicator]) {
        if !self.enabled {
            return;
        }

        // Disable raw mode and show cursor, but keep the display
        let _ = disable_raw_mode();
        let mut stdout = stdout();

        // Move cursor below the chart
        let _ = execute!(stdout, MoveTo(0, (self.top_n + 8) as u16), Show);

        if population.is_empty() {
            println!("\n❌ Population extinct!");
            return;
        }

        println!("\n");
        println!("════════════════════════════════════════════════════════════════");
        println!("                    FINAL POPULATION                            ");
        println!("════════════════════════════════════════════════════════════════");

        // Calculate final stats
        let n = population.len() as f64;
        let survival_mean = population.iter().map(|r| r.survival_rate).sum::<f64>() / n;
        let replication_mean = population.iter().map(|r| r.replication_rate).sum::<f64>() / n;
        let mutation_mean = population.iter().map(|r| r.mutation_rate).sum::<f64>() / n;
        let aggression_mean = population.iter().map(|r| r.aggression).sum::<f64>() / n;
        let age_mean = population.iter().map(|r| r.age as f64).sum::<f64>() / n;

        println!("Final population size:    {}", population.len());
        println!("Average survival rate:    {:.6}", survival_mean);
        println!("Average replication rate: {:.6}", replication_mean);
        println!("Average mutation rate:    {:.6}", mutation_mean);
        println!("Average aggression:       {:.6}", aggression_mean);
        println!("Average age:              {:.4}", age_mean);
        println!("════════════════════════════════════════════════════════════════");
    }

    /// Cleanup terminal on drop
    pub fn cleanup(&self) {
        if self.enabled {
            let _ = disable_raw_mode();
            let _ = execute!(stdout(), Show);
        }
    }
}

impl Drop for LiveVisualizer {
    fn drop(&mut self) {
        self.cleanup();
    }
}

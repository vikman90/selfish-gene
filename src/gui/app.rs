//! Main eframe application for Selfish Gene simulator.

use crate::config::Config;
use crate::gui::panels::controls::{render_controls_bar, ControlAction};
use crate::gui::panels::params::render_params_panel;
use crate::gui::panels::profiles::render_profiles_panel;
use crate::gui::panels::telemetry::render_telemetry_panel;
use crate::gui::panels::time_series::{render_time_series_panel, SeriesToggles};
use crate::gui::state::{ActiveTab, RunnerState, SimulationSpeed};
use crate::simulation::Simulation;
use eframe::App;

/// The primary application struct holding UI state and the simulation model
pub struct SelfishGeneApp {
    pub config: Config,
    pub use_seed: bool,
    pub seed_input: u64,
    pub simulation: Simulation,
    pub runner_state: RunnerState,
    pub speed: SimulationSpeed,
    pub active_tab: ActiveTab,
    pub series_toggles: SeriesToggles,
    pub export_filename: String,
    pub export_status: Option<(String, bool)>,
}

impl SelfishGeneApp {
    pub fn new(config: Config) -> Self {
        let use_seed = config.seed.is_some();
        let seed_input = config.seed.unwrap_or(42);
        let simulation = Simulation::with_config(config.clone());

        Self {
            config,
            use_seed,
            seed_input,
            simulation,
            runner_state: RunnerState::Idle,
            speed: SimulationSpeed::default(),
            active_tab: ActiveTab::TimeSeries,
            series_toggles: SeriesToggles::default(),
            export_filename: "simulation_results.json".to_string(),
            export_status: None,
        }
    }

    fn reset_simulation(&mut self) {
        if self.use_seed {
            self.config.seed = Some(self.seed_input);
        } else {
            self.config.seed = None;
        }
        self.simulation.reset(self.config.clone());
        self.runner_state = RunnerState::Idle;
        self.export_status = None;
    }
}

impl Default for SelfishGeneApp {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl App for SelfishGeneApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Advance Simulation if Running ---
        if self.runner_state.is_running() {
            for _ in 0..self.speed.steps_per_frame {
                if self.simulation.is_finished() {
                    if self.simulation.has_converged() {
                        self.runner_state = RunnerState::Converged;
                    } else if self.simulation.config().max_timesteps > 0
                        && self.simulation.timestep() >= self.simulation.config().max_timesteps
                    {
                        self.runner_state = RunnerState::MaxTimestepsReached;
                    }
                    break;
                }
                self.simulation.step();
            }
            ctx.request_repaint();
        }

        // --- Top Control Panel ---
        egui::TopBottomPanel::top("top_controls_panel")
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(action) = render_controls_bar(
                    ui,
                    self.runner_state,
                    &mut self.speed,
                    &mut self.active_tab,
                ) {
                    match action {
                        ControlAction::Start => self.runner_state = RunnerState::Running,
                        ControlAction::Pause => self.runner_state = RunnerState::Paused,
                        ControlAction::Step => {
                            if !self.simulation.is_finished() {
                                self.simulation.step();
                                if self.simulation.has_converged() {
                                    self.runner_state = RunnerState::Converged;
                                } else if self.simulation.config().max_timesteps > 0
                                    && self.simulation.timestep()
                                        >= self.simulation.config().max_timesteps
                                {
                                    self.runner_state = RunnerState::MaxTimestepsReached;
                                }
                            }
                        }
                        ControlAction::Reset => self.reset_simulation(),
                    }
                }
            });

        // --- Left Sidebar: Parameters ---
        egui::SidePanel::left("left_params_panel")
            .resizable(true)
            .default_width(280.0)
            .width_range(240.0..=450.0)
            .show(ctx, |ui| {
                let mut reset_requested = false;
                render_params_panel(
                    ui,
                    &mut self.config,
                    &mut self.use_seed,
                    &mut self.seed_input,
                    &mut reset_requested,
                );
                if reset_requested {
                    self.reset_simulation();
                }
            });

        // --- Central Visualization Area ---
        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            ActiveTab::TimeSeries => {
                render_time_series_panel(
                    ui,
                    self.simulation.history(),
                    self.config.capacity,
                    &mut self.series_toggles,
                );
            }
            ActiveTab::Profiles => {
                render_profiles_panel(ui, self.simulation.population(), self.config.top_profiles);
            }
            ActiveTab::Telemetry => {
                render_telemetry_panel(
                    ui,
                    &self.simulation,
                    &mut self.export_filename,
                    &mut self.export_status,
                );
            }
        });
    }
}

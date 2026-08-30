//! Simulation runner state and GUI view models.

use serde::{Deserialize, Serialize};

/// Current execution state of the simulation runner in the GUI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunnerState {
    /// Initial state, ready to start
    Idle,
    /// Simulation is running continuously
    Running,
    /// Simulation is temporarily paused by the user
    Paused,
    /// Simulation completed because genetic convergence was detected
    Converged,
    /// Simulation completed because max_timesteps was reached
    MaxTimestepsReached,
    /// Interrupted manually
    Interrupted,
}

impl RunnerState {
    pub fn is_running(&self) -> bool {
        matches!(self, RunnerState::Running)
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            RunnerState::Idle => "Listo",
            RunnerState::Running => "Ejecutando...",
            RunnerState::Paused => "Pausado",
            RunnerState::Converged => "Convergido (ESS)",
            RunnerState::MaxTimestepsReached => "Límite alcanzado",
            RunnerState::Interrupted => "Interrumpido",
        }
    }
}

/// Simulation speed configuration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimulationSpeed {
    /// Number of simulation steps to advance per UI frame
    pub steps_per_frame: usize,
    /// Throttle delay in milliseconds between frames (0 for maximum speed)
    pub frame_delay_ms: u64,
}

impl Default for SimulationSpeed {
    fn default() -> Self {
        Self {
            steps_per_frame: 1,
            frame_delay_ms: 0,
        }
    }
}

/// Active tab in the main visualization area
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ActiveTab {
    #[default]
    TimeSeries,
    Profiles,
    Telemetry,
}

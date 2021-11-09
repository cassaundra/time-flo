use std::time::{Duration, Instant};

use eframe::{egui, epi};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Timer {
    /// The total duration of this timer
    #[serde(with = "serde_millis")]
    duration: Duration,
    /// The amount of time which has been accumulated.
    /// This only increases when the timer has been paused.
    #[serde(with = "serde_millis")]
    accumulated_time: Duration,
    /// The time at which this timer was started.
    #[serde(with = "serde_millis")]
    start_timestamp: Option<Instant>,
}

impl Timer {
    pub fn from_duration(duration: Duration) -> Self {
        Self {
            duration,
            accumulated_time: Duration::ZERO,
            start_timestamp: None,
        }
    }

    pub fn start(&mut self) {
        self.start_timestamp = Some(Instant::now());
    }

    pub fn pause(&mut self) {
        if let Some(start_timestamp) = self.start_timestamp {
            let elapsed = Instant::now().duration_since(start_timestamp);
            self.accumulated_time += elapsed;
        }

        self.start_timestamp = None;
    }

    pub fn remaining_time(&self) -> Duration {
        return self.duration - self.elapsed();
    }

    pub fn is_over(&self) -> bool {
        return self.elapsed() >= self.duration;
    }

    pub fn is_paused(&self) -> bool {
        self.start_timestamp.is_none()
    }

    fn elapsed(&self) -> Duration {
        let current_elapsed = match self.start_timestamp {
            Some(start_timestamp) => {
                Instant::now().duration_since(start_timestamp)
            }
            None => Duration::ZERO,
        };
        self.accumulated_time + current_elapsed
    }
}

#[derive(Deserialize, Serialize)]
enum ProgramState {
    Idle,
    Task(Timer),
    ShortBreak(Timer),
    LongBreak(Timer),
}

impl ProgramState {
    /// Retrieve the duration of this interval as defined by the user preferences.
    pub fn duration(&self, preferences: &Preferences) -> f32 {
        match self {
            Idle => f32::INFINITY,
            Task => preferences.task_duration,
            ShortBreak => preferences.short_break_duration,
            LongBreak => preferences.long_break_duration,
        }
    }

    pub fn interval(&self) -> Option<&Timer> {
        use ProgramState::*;

        match self {
            Task(interval) | ShortBreak(interval) | LongBreak(interval) => {
                Some(interval)
            }
            _ => None,
        }
    }

    pub fn interval_mut(&mut self) -> Option<&mut Timer> {
        use ProgramState::*;

        match self {
            Task(interval) | ShortBreak(interval) | LongBreak(interval) => {
                Some(interval)
            }
            _ => None,
        }
    }
}

/// State of the TimeFlo program.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TimeFloApp {
    /// User-defined preferences.
    preferences: Preferences,
    /// The current state of the program.
    state: ProgramState,
}

impl Default for TimeFloApp {
    fn default() -> Self {
        Self {
            state: ProgramState::Idle,
            ..Default::default()
        }
    }
}

/// Preferences set by the user.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Preferences {
    /// Whether or not to display the user interface in dark mode.
    dark_mode: bool,
    /// Duration of a task interval in minutes.
    task_duration: f32,
    /// Duration of a short break in minutes.
    short_break_duration: f32,
    /// Duration of a long break in minutes.
    long_break_duration: f32,
    /// Number of short breaks before a long break.
    num_short_breaks: u32,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            dark_mode: true,
            task_duration: 25.,
            short_break_duration: 5.,
            long_break_duration: 15.,
            num_short_breaks: 3,
        }
    }
}

impl epi::App for TimeFloApp {
    fn name(&self) -> &str {
        "TimeFlo"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { prefs, state } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("TimeFlo");
            });
            ui.label("Watch this space!");

            egui::warn_if_debug_build(ui);
        });
    }
}

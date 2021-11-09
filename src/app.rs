use std::time::{Duration, Instant};

use eframe::{egui, epi};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize)]
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
    pub fn start(duration: Duration) -> Self {
        Self {
            duration,
            accumulated_time: Duration::ZERO,
            start_timestamp: Some(Instant::now()),
        }
    }

    pub fn resume(&mut self) {
        if self.start_timestamp.is_none() {
            self.start_timestamp = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        if let Some(start_timestamp) = self.start_timestamp {
            let elapsed = Instant::now().duration_since(start_timestamp);
            self.accumulated_time += elapsed;
        }

        self.start_timestamp = None;
    }

    pub fn remaining_time(&self) -> Duration {
        return self.duration.saturating_sub(self.elapsed());
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

#[derive(PartialEq, Deserialize, Serialize)]
enum ProgramState {
    Idle,
    Task(Timer),
    ShortBreak(Timer),
    LongBreak(Timer),
}

impl ProgramState {
    pub fn task(preferences: &Preferences) -> ProgramState {
        ProgramState::Task(Timer::start(preferences.task_duration()))
    }

    pub fn short_break(preferences: &Preferences) -> ProgramState {
        ProgramState::ShortBreak(Timer::start(
            preferences.short_break_duration(),
        ))
    }

    pub fn long_break(preferences: &Preferences) -> ProgramState {
        ProgramState::LongBreak(Timer::start(preferences.long_break_duration()))
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
    /// Number of short breaks which have occurred since the last long break, or
    /// the start of the program.
    short_break_counter: u32,
}

impl Default for TimeFloApp {
    fn default() -> Self {
        Self {
            state: ProgramState::Idle,
            preferences: Preferences::default(),
            short_break_counter: 0,
        }
    }
}

impl TimeFloApp {
    fn next_state(&mut self) -> ProgramState {
        use ProgramState::*;

        match self.state {
            Idle | ShortBreak(_) | LongBreak(_) => {
                ProgramState::task(&self.preferences)
            }
            Task(_) => {
                if self.short_break_counter < self.preferences.num_short_breaks
                {
                    self.short_break_counter += 1;
                    ProgramState::short_break(&self.preferences)
                } else {
                    self.short_break_counter = 0;
                    ProgramState::long_break(&self.preferences)
                }
            }
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
    task_minutes: f32,
    /// Duration of a short break in minutes.
    short_break_minutes: f32,
    /// Duration of a long break in minutes.
    long_break_minutes: f32,
    /// Number of short breaks before a long break.
    num_short_breaks: u32,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            dark_mode: true,
            task_minutes: 25.,
            short_break_minutes: 5.,
            long_break_minutes: 15.,
            num_short_breaks: 3,
        }
    }
}

impl Preferences {
    pub fn task_duration(&self) -> Duration {
        Duration::from_secs_f32(self.task_minutes * 60.)
    }

    pub fn short_break_duration(&self) -> Duration {
        Duration::from_secs_f32(self.short_break_minutes * 60.)
    }

    pub fn long_break_duration(&self) -> Duration {
        Duration::from_secs_f32(self.long_break_minutes * 60.)
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
        if self.state != ProgramState::Idle {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("TimeFlo");
            });

            let format_with_time = |name: &str, timer: &Timer| {
                format!(
                    "{} ({:.1} remaining)",
                    name,
                    timer.remaining_time().as_secs_f32()
                )
            };

            let state_label = match &mut self.state {
                ProgramState::Idle => String::from("Idle"),
                ProgramState::Task(timer) => format_with_time("Task", timer),
                ProgramState::ShortBreak(timer) => {
                    format_with_time("Short break", timer)
                }
                ProgramState::LongBreak(timer) => {
                    format_with_time("Long break", timer)
                }
            };
            ui.label(&format!("State: {}", state_label));

            match &mut self.state {
                ProgramState::Idle => {
                    if ui.button("Start").clicked() {
                        self.state = self.next_state();
                    }
                }
                ProgramState::Task(timer)
                | ProgramState::ShortBreak(timer)
                | ProgramState::LongBreak(timer) => {
                    if !timer.is_paused() {
                        if ui.button("Pause").clicked() {
                            timer.pause();
                        }
                    } else {
                        if ui.button("Resume").clicked() {
                            timer.resume();
                        }
                    }

                    if ui.button("Skip").clicked() || timer.is_over() {
                        self.state = self.next_state();
                    }
                }
            }

            egui::warn_if_debug_build(ui);
        });
    }
}

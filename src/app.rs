use std::time::{Duration, Instant};

use eframe::{
    egui::{self, Color32},
    epi,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, PartialEq, Deserialize, Serialize)]
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

    pub fn elapsed(&self) -> Duration {
        let current_elapsed = match self.start_timestamp {
            Some(start_timestamp) => {
                Instant::now().duration_since(start_timestamp)
            }
            None => Duration::ZERO,
        };
        self.accumulated_time + current_elapsed
    }

    pub fn remaining_time(&self) -> Duration {
        return self.duration.saturating_sub(self.elapsed());
    }

    pub fn has_started(&self) -> bool {
        return self.elapsed() > Duration::ZERO;
    }

    pub fn is_over(&self) -> bool {
        return self.elapsed() >= self.duration;
    }

    pub fn is_running(&self) -> bool {
        self.start_timestamp.is_some()
    }

    pub fn is_paused(&self) -> bool {
        self.start_timestamp.is_none() && self.has_started()
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Idle,
    Task,
    ShortBreak,
    LongBreak,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

impl State {
    pub fn is_break(&self) -> bool {
        return *self == State::ShortBreak || *self == State::LongBreak;
    }
}

/// State of the TimeFlo program.
#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct TimeFloApp {
    /// User-defined preferences.
    preferences: Preferences,
    /// The current state of the program.
    state: State,
    /// The underlying timer.
    timer: Timer,
    /// Number of short breaks which have occurred since the last long break, or
    /// the start of the program.
    short_break_counter: u32,
}

impl TimeFloApp {
    fn change_state(&mut self, state: State) {
        self.state = state;
        self.timer = Timer::from_duration(
            self.preferences.preferred_duration(self.state),
        );

        // if a break, start the timer immediately
        if state.is_break() {
            self.timer.start();
        }

        // update break counter
        match state {
            State::ShortBreak => self.short_break_counter += 1,
            State::LongBreak => self.short_break_counter = 0,
            _ => {}
        }
    }

    fn next_state(&self) -> State {
        match self.state {
            State::Task => {
                // is it time for a long break?
                if self.short_break_counter < self.preferences.num_short_breaks
                {
                    State::ShortBreak
                } else {
                    State::LongBreak
                }
            }
            _ => State::Task,
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
    pub fn preferred_duration(&self, state: State) -> Duration {
        let minutes = match state {
            State::Idle => 0.,
            State::Task => self.task_minutes,
            State::ShortBreak => self.short_break_minutes,
            State::LongBreak => self.long_break_minutes,
        };
        Duration::from_secs_f32(minutes * 60.)
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

        self.change_state(State::Task);
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        if self.timer.is_running() {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&format!(
                "{:?} ({:.0}s)",
                self.state,
                self.timer.remaining_time().as_secs_f32(),
            ));

            ui.separator();

            ui.horizontal_wrapped(|ui| {
                let timer = &mut self.timer;
                if !timer.has_started() {
                    let begin_button = ui.add(
                        egui::Button::new("Begin")
                            .fill(Color32::BLUE)
                            .stroke((1., Color32::DARK_BLUE)),
                    );
                    if begin_button.clicked() {
                        timer.start();
                    }
                } else if timer.is_paused() {
                    if ui.button("▶").clicked() {
                        timer.start();
                    }
                } else {
                    if ui.button("⏸").clicked() {
                        timer.pause();
                    }
                }

                if self.state.is_break() || timer.has_started() {
                    if ui.button("⏭").clicked() || timer.is_over() {
                        self.change_state(self.next_state());
                    }
                }
            });

            egui::warn_if_debug_build(ui);
        });
    }
}

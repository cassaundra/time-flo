use std::fmt;
use std::time::{Duration, Instant};

use eframe::{
    egui::{self, Color32},
    epi,
};

use notify_rust::Notification;
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

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_seconds = self.remaining_time().as_secs();
        write!(f, "{:02}:{:02}", total_seconds / 60, total_seconds % 60)
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Idle,
    Task,
    ShortBreak,
    LongBreak,
}

impl State {
    pub fn is_break(&self) -> bool {
        return *self == State::ShortBreak || *self == State::LongBreak;
    }
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            State::Idle => "Idle",
            State::Task => "Task period",
            State::ShortBreak => "Short break",
            State::LongBreak => "Long break",
        };

        write!(f, "{}", name)
    }
}

/// Preferences set by the user.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Preferences {
    /// Duration of a task interval in minutes.
    task_minutes: f32,
    /// Duration of a short break in minutes.
    short_break_minutes: f32,
    /// Duration of a long break in minutes.
    long_break_minutes: f32,
    /// Number of short breaks before a long break.
    num_short_breaks: u32,
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

impl Default for Preferences {
    fn default() -> Self {
        Self {
            task_minutes: 25.,
            short_break_minutes: 5.,
            long_break_minutes: 15.,
            num_short_breaks: 3,
        }
    }
}

/// State of the TimeFlo program.
#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct TimeFloApp {
    /// User-defined preferences.
    preferences: Preferences,
    /// The current state of the program.
    #[serde(skip)]
    state: State,
    /// The underlying timer.
    #[serde(skip)]
    timer: Timer,
    /// Number of short breaks which have occurred since the last long break, or
    /// the start of the program.
    #[serde(skip)]
    short_break_counter: u32,
    /// Whether or not the preferences dialog is visible
    #[serde(skip)]
    preferences_visible: bool,
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

    fn main_view(&mut self, ui: &mut egui::Ui) {
        ui.heading(&format!("{}", self.state));
        ui.monospace(&format!("{}", self.timer));

        ui.separator();

        ui.horizontal(|ui| {
            let timer = &mut self.timer;
            if !timer.has_started() {
                // this will realistically only be shown when the program is
                // in "task" mode, because all others automatically start
                // the timer

                let begin_button = ui.add(
                    egui::Button::new("Begin task")
                        .fill(Color32::BLUE)
                        .stroke((1., Color32::DARK_BLUE)),
                );

                if begin_button.clicked() {
                    timer.start();
                }
            } else if timer.is_paused() {
                if ui.button("Resume").clicked() {
                    timer.start();
                }
            } else {
                // the timer is currently running
                if ui.button("Pause").clicked() {
                    timer.pause();
                }
            }

            // show a skip button for breaks
            if self.state.is_break() || timer.has_started() {
                if ui.button("Skip").clicked() {
                    self.change_state(self.next_state());
                }
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            // gear icon
            if ui.button("\u{2699}").clicked() {
                self.preferences_visible = true;
            }
        });
    }

    fn preferences_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("Preferences");

        ui.separator();

        let prefs = &mut self.preferences;

        // TODO reduce code reuse
        ui.add(
            egui::Slider::new(&mut prefs.task_minutes, 1.0..=120.0)
                .suffix(" min")
                .text("Task period"),
        );
        ui.add(
            egui::Slider::new(&mut prefs.short_break_minutes, 0.0..=120.0)
                .suffix(" min")
                .text("Short break"),
        );
        ui.add(
            egui::Slider::new(&mut prefs.long_break_minutes, 0.0..=120.0)
                .suffix(" min")
                .text("Long break"),
        );

        ui.separator();

        ui.add(
            egui::Slider::new(&mut prefs.num_short_breaks, 1..=16)
                .text("Short breaks"),
        );

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Reset to default").clicked() {
                self.preferences = Preferences::default();
            }

            if ui.button("Close").clicked() {
                self.preferences_visible = false;
            }
        });
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

        // has the timer just complete?
        if self.timer.is_over() {
            // this is a little hacky, but it works!
            let state_name = format!("{}", self.state).to_lowercase();

            // notify the user
            // TODO handle result
            Notification::new()
                .summary("TimeFlo")
                .body(&format!("Your {} is over!", state_name))
                .timeout(5000)
                .show()
                .expect("Could not display notification");

            // change to the next
            self.change_state(self.next_state());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.preferences_visible {
                self.preferences_view(ui);
            } else {
                self.main_view(ui);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        // test basic operations

        let t1 = Timer::from_duration(Duration::ZERO);

        assert!(!t1.is_paused());
        assert!(!t1.is_running());
        assert!(!t1.has_started());

        let mut t2 = Timer {
            duration: Duration::from_secs(20),
            accumulated_time: Duration::from_secs(12),
            start_timestamp: None,
        };

        assert!(t2.is_paused());
        assert!(!t2.is_running());
        assert!(t2.has_started());
        assert_eq!(Duration::from_secs(8), t2.elapsed());

        t2.start();
        assert!(!t2.is_paused());
        assert!(t2.is_running());
        assert!(t2.has_started());

        // test formatting
        assert_eq!("00:00", format!("{}", Timer::default()));
        assert_eq!(
            "12:34",
            format!("{}", Timer::from_duration(Duration::from_secs(754)))
        );
    }

    #[test]
    fn test_app() {
        // test an aspect state changing
        // TODO more!

        let mut app = TimeFloApp {
            state: State::Task,
            short_break_counter: 3,
            preferences: Preferences {
                num_short_breaks: 3,
                ..Default::default()
            },
            ..Default::default()
        };

        app.change_state(app.next_state());

        assert_eq!(State::LongBreak, app.state);
        assert!(app.timer.has_started());
    }
}

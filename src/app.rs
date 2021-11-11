use std::fmt;
use std::time::Duration;

use eframe::{
    egui::{self, Color32},
    epi,
};
use notify_rust::Notification;
use serde::{Deserialize, Serialize};

use crate::timer::Timer;

macro_rules! slider {
    ($ui:ident, $val:expr, $name:expr, $range:expr) => {
        $ui.add(::eframe::egui::Slider::new(&mut $val, $range).text($name));
    };
    ($ui:ident, $val:expr, $name:expr, $range:expr, $suffix:expr) => {
        $ui.add(
            ::eframe::egui::Slider::new(&mut $val, $range)
                .text($name)
                .suffix($suffix),
        );
    };
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
        match self {
            State::ShortBreak | State::LongBreak => true,
            _ => false,
        }
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
struct Preferences {
    /// Duration of a task interval in minutes.
    pub task_minutes: f32,
    /// Duration of a short break in minutes.
    pub short_break_minutes: f32,
    /// Duration of a long break in minutes.
    pub long_break_minutes: f32,
    /// Number of short breaks before a long break.
    pub num_short_breaks: u32,
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

    // called when preferences have changed
    fn update_preferences(&mut self) {
        // update timer duration according to preferences
        self.timer
            .set_duration(self.preferences.preferred_duration(self.state));
    }

    fn main_view(&mut self, ui: &mut egui::Ui) {
        ui.heading(format!("{}", self.state));

        let timer_color = if self.timer.remaining_time().as_secs() <= 5 {
            Color32::RED
        } else {
            ui.visuals().text_color()
        };

        ui.add(
            egui::Label::new(format!("{}", self.timer))
                .monospace()
                .text_color(timer_color),
        );

        ui.separator();

        ui.horizontal(|ui| {
            let timer = &mut self.timer;

            if !self.state.is_break() && !timer.has_started() {
                // waiting for user to begin task

                let begin_button = ui.add(
                    egui::Button::new("Begin task")
                        .fill(Color32::BLUE)
                        .stroke((1., Color32::DARK_BLUE)),
                );

                if begin_button.clicked() {
                    timer.start();
                }
            } else if timer.is_paused() {
                // the timer is paused
                if ui.button("Resume").clicked() {
                    timer.start();
                }
            } else {
                // the timer is currently running
                if ui.button("Pause").clicked() {
                    timer.pause();
                }
            }

            // show a skip button for breaks, or if the timer is running
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

        ui.collapsing("Interval durations", |ui| {
            slider!(ui, prefs.task_minutes, "Task period", 0.5..=120.0);
            slider!(ui, prefs.short_break_minutes, "Short break", 0.5..=120.0);
            slider!(ui, prefs.long_break_minutes, "Long break", 0.5..=120.0);
        });

        ui.collapsing("Program flow", |ui| {
            slider!(ui, prefs.num_short_breaks, "Short breaks", 1..=16);
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Reset to default").clicked() {
                self.preferences = Preferences::default();
            }

            if ui.button("Close").clicked() {
                self.update_preferences();
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

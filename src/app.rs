use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use rodio::Source;
use wsl;

pub enum PomodoroStage {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct Interruption {
    pub started_at: Instant,
    pub finished_at: Option<Instant>,
    pub annotation: Option<String>,
}

impl Clone for Interruption {
    fn clone(&self) -> Interruption {
        Interruption {
            started_at: self.started_at.clone(),
            finished_at: self.finished_at.clone(),
            annotation: self.annotation.clone(),
        }
    }
}

impl Interruption {
    fn new(started_at: Instant) -> Interruption {
        Interruption {
            started_at: started_at,
            finished_at: None,
            annotation: None,
        }
    }
}

pub struct PomodoroCycle {
    stage_iteration: usize,
    started_at: Option<Instant>,
    finished_at: Option<Instant>,
    interruptions_history: Vec<Interruption>,
    interruption: Option<Interruption>,
}

impl Clone for PomodoroCycle {
    fn clone(&self) -> PomodoroCycle {
        PomodoroCycle {
            stage_iteration: self.stage_iteration,
            started_at: self.started_at.clone(),
            finished_at: self.finished_at.clone(),
            interruptions_history: self.interruptions_history.clone(),
            interruption: self.interruption.clone(),
        }
    }
}

impl PomodoroCycle {
    pub fn new(stage_iteration: usize) -> PomodoroCycle {
        PomodoroCycle {
            stage_iteration: stage_iteration,
            started_at: None,
            finished_at: None,
            interruptions_history: Vec::new(),
            interruption: None,
        }
    }
}

pub struct AppConfiguration {
    work_duration: Duration,
    short_break_duration: Duration,
    long_break_duration: Duration,
    stage_sequence: [PomodoroStage; 8],
}

impl Default for AppConfiguration {
    fn default() -> AppConfiguration {
        AppConfiguration {
            work_duration: Duration::new(1500, 0),
            short_break_duration: Duration::new(300, 0),
            long_break_duration: Duration::new(1200, 0),
            // TODO: make sequence configurable
            stage_sequence: [
                PomodoroStage::Work,
                PomodoroStage::ShortBreak,
                PomodoroStage::Work,
                PomodoroStage::ShortBreak,
                PomodoroStage::Work,
                PomodoroStage::ShortBreak,
                PomodoroStage::Work,
                PomodoroStage::LongBreak,
            ],
        }
    }
}

pub enum AppView {
    Normal,
    AnnotationPopup,
    InterruptionsList,
}

pub struct App {
    view: AppView,
    config: AppConfiguration,
    current_cycle: PomodoroCycle,
    history: Vec<PomodoroCycle>,
}

impl Default for App {
    fn default() -> App {
        App {
            view: AppView::Normal,
            config: AppConfiguration::default(),
            current_cycle: PomodoroCycle {
                stage_iteration: 0,
                started_at: None,
                finished_at: None,
                interruptions_history: Vec::new(),
                interruption: None,
            },
            history: Vec::new(),
        }
    }
}

impl App {
    pub fn change_view(&mut self, view: AppView) {
        self.view = view
    }

    pub fn get_view(&self) -> &AppView {
        &self.view
    }

    pub fn is_paused(&self) -> bool {
        self.current_cycle.interruption.is_some()
    }

    pub fn get_pause_elapsed_time(&self) -> u64 {
        match self.current_cycle.interruption.as_ref() {
            Some(interruption) => (Instant::now() - interruption.started_at).as_secs(),
            None => 0,
        }
    }

    pub fn append_to_interruption_annotation(&mut self, c: char) {
        self.current_cycle
            .interruption
            .as_mut()
            .and_then(|interruption| match interruption.annotation.as_mut() {
                Some(annotation) => {
                    annotation.push(c);
                    Some(())
                }
                None => {
                    interruption.annotation = Some(c.to_string());
                    Some(())
                }
            });
    }

    pub fn pop_from_interruption_annotation(&mut self) {
        self.current_cycle
            .interruption
            .as_mut()
            .and_then(|interruption| match interruption.annotation.as_mut() {
                Some(annotation) => {
                    annotation.pop();
                    Some(())
                }
                None => None,
            });
    }

    pub fn get_interruption_annotation(&self) -> Option<String> {
        match self.current_cycle.interruption.as_ref() {
            Some(interruption) => interruption.annotation.clone(),
            None => None,
        }
    }

    pub fn get_interruption_history(&self) -> &Vec<Interruption> {
        &self.current_cycle.interruptions_history
    }

    pub fn get_current_stage(&self) -> &PomodoroStage {
        let idx = self.current_cycle.stage_iteration % self.config.stage_sequence.len();
        &self.config.stage_sequence[idx]
    }

    /// Toggles the timer.
    /// This is the primary method for interacting and manipulating the timer.
    /// It can:
    ///
    /// - Start the timer if it wasn't started yet
    /// - Pause the timer if it is currently running
    /// - Resume the timer if it is currently paused
    pub fn toggle_timer(&mut self) {
        let toggled_at = Instant::now();

        if self.current_cycle.started_at.is_none() {
            self.current_cycle.started_at = Some(toggled_at);
            return;
        }

        if self.current_cycle.interruption.is_none() {
            self.current_cycle.interruption = Some(Interruption::new(toggled_at));
            self.view = AppView::AnnotationPopup;
        } else {
            let mut interruption = self.current_cycle.interruption.take().unwrap();
            interruption.finished_at = Some(toggled_at);
            self.current_cycle.interruptions_history.push(interruption);
        }
    }

    /// Calculates the elapsed duration of the current pomodoro stage.
    /// There are four possible scenarios to deal with when performing the calculation:
    ///
    /// 1. The timer hasn't started
    /// 2. The timer has started, but there were no pauses yet
    /// 3. The timer has started, there were pauses, but not currently paused
    /// 4. The timer has started, there were pauses, is currently paused
    fn get_elapsed_time(&self) -> Duration {
        if self.current_cycle.started_at.is_none() {
            return Duration::new(0, 0);
        }

        let started_at = self.current_cycle.started_at.unwrap();

        if self.current_cycle.interruptions_history.len() == 0
            && self.current_cycle.interruption.is_none()
        {
            // There were no interruptions up to this point
            // so its straight-forward to calculate the elapsed time
            return Instant::now() - started_at;
        }

        let total_elapsed_on_pauses: Duration = self
            .current_cycle
            .interruptions_history
            .iter()
            .fold(Duration::new(0, 0), |total, interruption| {
                let elapsed = match interruption.finished_at {
                    Some(finished_at) => (finished_at - interruption.started_at),
                    None => Duration::new(0, 0),
                };
                total + elapsed
            });

        let was_last_active_at = match self.current_cycle.interruption.as_ref() {
            Some(interruption) => interruption.started_at,
            None => Instant::now(),
        };

        (was_last_active_at - started_at) - total_elapsed_on_pauses
    }

    /// Returns whether the currently configured timer is due
    /// and the time left as a formatted string
    /// # Examples
    ///
    /// ```
    /// let (is_due, remaining_time) = app.get_remaining_time();
    /// ```
    ///
    pub fn get_remaining_time(&mut self) -> (bool, String) {
        let duration = match self.get_current_stage() {
            PomodoroStage::Work => self.config.work_duration,
            PomodoroStage::ShortBreak => self.config.short_break_duration,
            PomodoroStage::LongBreak => self.config.long_break_duration,
        };

        let elapsed = self.get_elapsed_time();

        if elapsed >= duration {
            if self.current_cycle.finished_at.is_none() {
                self.current_cycle.finished_at = Some(Instant::now());
            }
            return (true, String::from("00:00"));
        }

        let remaining = (duration - elapsed).as_secs();

        let minutes = remaining / 60;
        let seconds = remaining % 60;

        (false, format!("{:02}:{:02}", minutes, seconds))
    }

    pub fn finish_current_cycle(&mut self) {
        if self.current_cycle.finished_at.is_none() {
            self.current_cycle.finished_at = Some(Instant::now());
        }

        self.history.push(self.current_cycle.clone());
        self.current_cycle = PomodoroCycle::new(self.current_cycle.stage_iteration + 1);

        if !wsl::is_wsl() {
            if let Some(device) = rodio::default_output_device() {
                if let Ok(file) = File::open("sounds/beep.wav") {
                    if let Ok(src) = rodio::Decoder::new(BufReader::new(file)) {
                        rodio::play_raw(
                            &device,
                            src.take_duration(Duration::from_secs(1)).convert_samples(),
                        );
                    }
                }
            }
        }
    }
}
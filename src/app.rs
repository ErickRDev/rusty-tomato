use std::time::{Duration, Instant};

pub enum PomodoroStage {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct PomodoroCycle {
    stage_iteration: usize,
    started_at: Option<Instant>,
    finished_at: Option<Instant>,
    interruptions: Vec<(Instant, Option<Instant>)>,
    is_paused: bool,
}

impl Clone for PomodoroCycle {
    fn clone(&self) -> PomodoroCycle {
        PomodoroCycle {
            stage_iteration: self.stage_iteration,
            started_at: self.started_at.clone(),
            finished_at: self.finished_at.clone(),
            interruptions: self.interruptions.clone(),
            is_paused: self.is_paused,
        }
    }
}

impl PomodoroCycle {
    pub fn new(stage_iteration: usize) -> PomodoroCycle {
        PomodoroCycle {
            stage_iteration: stage_iteration,
            started_at: None,
            finished_at: None,
            interruptions: Vec::new(),
            is_paused: false,
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

pub struct App {
    config: AppConfiguration,
    current_cycle: PomodoroCycle,
    history: Vec<PomodoroCycle>,
}

impl Default for App {
    fn default() -> App {
        App {
            config: AppConfiguration::default(),
            current_cycle: PomodoroCycle {
                stage_iteration: 0,
                started_at: None,
                finished_at: None,
                interruptions: Vec::new(),
                is_paused: false,
            },
            history: Vec::new(),
        }
    }
}

impl App {
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

        let interruption: (Instant, Option<Instant>) = match self.current_cycle.is_paused {
            true => {
                let (paused_at, _) = self.current_cycle.interruptions.pop().unwrap();
                (paused_at, Some(toggled_at))
            }
            false => (toggled_at, None),
        };

        self.current_cycle.interruptions.push(interruption);
        self.current_cycle.is_paused = !self.current_cycle.is_paused;
    }

    /// Calculates the elapsed duration of the current pomodoro stage.
    /// There are four possible scenarios to deal with when performing the calculation:
    ///
    /// 1. The timer hasn't started
    /// 2. The timer has started, but there were no pauses yet
    /// 3. The timer has started, there were pauses, but not currently paused
    /// 4. The timer has started, there were pauses, is currently paused
    fn get_elapsed_time(&mut self) -> Duration {
        if self.current_cycle.started_at.is_none() {
            return Duration::new(0, 0);
        }

        let started_at = self.current_cycle.started_at.unwrap();

        if self.current_cycle.interruptions.len() == 0 {
            // There were no interruptions up to this point
            // so its straight-forward to calculate the elapsed time
            return Instant::now() - started_at;
        }

        let total_elapsed_on_pauses: Duration = self.current_cycle.interruptions.iter().fold(
            Duration::new(0, 0),
            |total, &(paused_at, resumed_at)| {
                let elapsed = match resumed_at {
                    Some(instant) => (instant - paused_at),
                    None => Duration::new(0, 0),
                };
                total + elapsed
            },
        );

        let was_last_active_at = match self.current_cycle.is_paused {
            true => {
                let interruption = self.current_cycle.interruptions.pop().unwrap();
                let paused_at = interruption.0.clone();
                self.current_cycle.interruptions.push(interruption);
                paused_at
            }
            false => Instant::now(),
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
    }
}

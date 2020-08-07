use std::time::{Duration, Instant};

pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct AppConfiguration {
    work_duration: Duration,
    short_break_duration: Duration,
    long_break_duration: Duration,
}

impl Default for AppConfiguration {
    fn default() -> AppConfiguration {
        AppConfiguration {
            // work_duration: Duration::new(1500, 0),
            work_duration: Duration::new(10, 0),
            short_break_duration: Duration::new(300, 0),
            long_break_duration: Duration::new(1200, 0),
        }
    }
}

// impl AppConfiguration {
//     fn configure_work_duration(mut self, seconds: u64) -> AppConfiguration {
//         self.work_duration = Duration::new(seconds, 0);
//         self
//     }

//     fn configure_short_break_duration(mut self, seconds: u64) -> AppConfiguration {
//         self.short_break_duration = Duration::new(seconds, 0);
//         self
//     }

//     fn configure_long_break_duration(mut self, seconds: u64) -> AppConfiguration {
//         self.long_break_duration = Duration::new(seconds, 0);
//         self
//     }
// }

pub struct App {
    config: AppConfiguration,
    current_phase: PomodoroPhase,
    started_at: Option<Instant>,
    is_paused: bool,
}

impl Default for App {
    fn default() -> App {
        App {
            config: AppConfiguration::default(),
            current_phase: PomodoroPhase::Work,
            started_at: None,
            is_paused: true,
        }
    }
}

impl App {
    pub fn start_timer(&mut self) {
        self.started_at = Some(Instant::now());
    }

    pub fn pause_timer(&mut self) {
        self.started_at = None;
    }

    pub fn reset_timer(&mut self) {
        self.started_at = Some(Instant::now());
    }

    pub fn change_phase(&mut self, phase: PomodoroPhase) {
        self.current_phase = phase;
        self.reset_timer();
    }

    /// Returns whether the currently configured timer is due
    /// and the time left as a formatted string
    /// # Examples
    ///
    /// ```
    /// let (is_due, remaining_time) = app.get_remaining_time()
    pub fn get_remaining_time(&self) -> (bool, String) {
        let duration = match self.current_phase {
            PomodoroPhase::Work => self.config.work_duration,
            PomodoroPhase::ShortBreak => self.config.short_break_duration,
            PomodoroPhase::LongBreak => self.config.long_break_duration,
        };

        let elapsed = match self.started_at {
            Some(started_at) => Instant::now() - started_at,
            None => Duration::new(0, 0),
        };

        if elapsed >= duration {
            return (true, String::from("00:00"));
        }

        let remaining = (duration - elapsed).as_secs();

        let minutes = remaining / 60;
        let seconds = remaining % 60;

        (false, format!("{:02}:{:02}", minutes, seconds))
    }
}

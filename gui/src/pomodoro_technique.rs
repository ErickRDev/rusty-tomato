use std::time::Instant;

/// A pomodoro cycle.
pub struct Cycle {
    pub stage_iteration: usize,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub interruption_history: Vec<Interruption>,
    pub interruption: Option<Interruption>,
}

impl Clone for Cycle {
    fn clone(&self) -> Cycle {
        Cycle {
            stage_iteration: self.stage_iteration,
            started_at: self.started_at.clone(),
            finished_at: self.finished_at.clone(),
            interruption_history: self.interruption_history.clone(),
            interruption: self.interruption.clone(),
        }
    }
}

impl Cycle {
    pub fn new(stage_iteration: usize) -> Cycle {
        Cycle {
            stage_iteration,
            started_at: None,
            finished_at: None,
            interruption_history: Vec::new(),
            interruption: None,
        }
    }
}

/// Pomodoro stages.
pub enum Stage {
    Work,
    ShortBreak,
    LongBreak,
}

/// An interruption to a pomodoro stage.
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
    pub fn new(started_at: Instant) -> Interruption {
        Interruption {
            started_at,
            finished_at: None,
            annotation: None,
        }
    }
}

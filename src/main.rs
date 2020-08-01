pub mod widgets;

extern crate chrono;
use chrono::{DateTime, Utc};

use structopt::StructOpt;

use std::{
    error::Error,
    io::{stdout, Write},
};

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, read, Event, KeyCode},
    execute, terminal,
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Text},
    Terminal,
};

use crate::widgets::Clock;

#[derive(StructOpt, Debug)]
#[structopt(name = "pomodoro")]
struct Pomodoro {
    #[structopt(long)]
    debug: bool,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(short, long, default_value = "250")]
    tick_duration: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Pomodoro::from_args();
    println!("{:#?}", opts);

    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(opts.tick_duration)).unwrap() {
            if let Event::Key(key) = read().unwrap() {
                tx.send(key).unwrap();
            }
        }
    });

    loop {
        terminal.draw(|mut f| {
            let size = f.size();

            let block = Block::default()
                .style(Style::default().fg(Color::Yellow))
                .borders(Borders::ALL);
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(size);

            let clock = Clock::default();
            f.render_widget(clock, chunks[1]);

            // for ch in 0..chunks.len() {
            //     let clock = Clock::default();
            //     f.render_widget(clock, chunks[ch]);
            // }
        })?;

        match rx.recv()?.code {
            KeyCode::Char('q') => {
                execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
                return Ok(());
            }
            _ => {}
        }
    }
}

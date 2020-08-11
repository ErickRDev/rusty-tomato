pub mod app;
pub mod widgets;

use structopt::StructOpt;

use std::{
    error::Error,
    io::{stdout, Write},
    time::Duration,
};

use std::sync::mpsc;
use std::thread;

use crossterm::{
    event::{self, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
    Terminal,
};

use crate::app::App;
use crate::widgets::Timer;

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

enum TickContent {
    KeyPress(crossterm::event::KeyEvent),
    None,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Pomodoro::from_args();
    println!("{:#?}", opts);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(opts.tick_duration)).unwrap() {
            if let Event::Key(key) = read().unwrap() {
                tx.send(TickContent::KeyPress(key)).unwrap();
            }
        }

        tx.send(TickContent::None).unwrap();
    });

    let mut draw_borders = false;

    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(Clear, size);

            let block = Block::default()
                .style(Style::default().fg(Color::Yellow))
                .borders(Borders::ALL);

            if draw_borders {
                f.render_widget(block.clone(), size);
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(size);

            if draw_borders {
                f.render_widget(block.clone(), chunks[1]);
            }

            let (is_due, remaining_time) = app.get_remaining_time();

            if is_due {
                app.finish_current_cycle();
            }

            if app.is_paused() {
                // TODO: render paused widgets here
                // app.current_cycle.in
                // chunks[0];
            }

            let is_paused = app.is_paused();
            let elapsed_pause_time: Option<u64> = if is_paused {
                Some(app.get_pause_elapsed_time())
            } else {
                None
            };

            let clock = Timer::default()
                .time_remaining(&remaining_time)
                .borders(draw_borders)
                .is_paused(is_paused, elapsed_pause_time)
                .is_due(is_due);

            f.render_widget(clock, chunks[1]);
        })?;

        match rx.recv()? {
            TickContent::KeyPress(key_event) => match key_event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                }
                KeyCode::Char('c') => {
                    app.finish_current_cycle();
                }
                KeyCode::Char('d') => {
                    draw_borders = !draw_borders;
                }
                KeyCode::Char(' ') => {
                    app.toggle_timer();
                }
                _ => {}
            },
            TickContent::None => {}
        }
    }

    Ok(())
}

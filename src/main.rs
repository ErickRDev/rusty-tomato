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
    cursor,
    event::{self, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Terminal,
};

use crate::app::{App, AppView};
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
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
                .style(Style::default().fg(Color::Red))
                .borders(Borders::ALL);

            if draw_borders {
                f.render_widget(block.clone(), size);
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                        Constraint::Percentage(5),
                        Constraint::Percentage(15),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(size);

            let pause_timer_area = chunks[2];
            let pause_timer_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(pause_timer_area)[1];

            let pause_annotation_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(chunks[3])[1];

            if draw_borders {
                f.render_widget(block.clone(), pause_timer_area);
                f.render_widget(block.clone(), pause_annotation_area);
            }

            if app.is_paused() {
                let has_been_paused_for = app.get_pause_elapsed_time();

                let minutes = has_been_paused_for / 60;
                let seconds = has_been_paused_for % 60;

                let span = Text::from(Span::from(String::from(format!(
                    "{:02}:{:02}",
                    minutes, seconds
                ))));

                let paragraph = Paragraph::new(span).alignment(Alignment::Center);
                f.render_widget(paragraph, pause_timer_area);

                match app.get_view() {
                    AppView::AnnotationPopup => {
                        f.render_widget(block.clone(), pause_annotation_area);

                        match app.get_interruption_annotation() {
                            Some(annotation) => {
                                let annotation_length = annotation.len() as u16;
                                let span = Span::from(annotation);
                                let paragraph = Paragraph::new(span)
                                    .block(block.clone())
                                    .alignment(Alignment::Left);
                                f.render_widget(paragraph, pause_annotation_area);
                                f.set_cursor(
                                    pause_annotation_area.x + 1 + annotation_length,
                                    pause_annotation_area.y + 1,
                                );
                            }
                            None => {}
                        }
                    }
                    _ => {}
                }
            }

            let pomodoro_timer_area = chunks[1];
            let pomodoro_timer_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(pomodoro_timer_area)[1];

            if draw_borders {
                f.render_widget(block.clone(), pomodoro_timer_area);
            }

            let (is_due, remaining_time) = app.get_remaining_time();

            if is_due {
                app.finish_current_cycle();
            }

            let clock = Timer::default()
                .time_remaining(&remaining_time)
                .borders(draw_borders)
                .is_paused(app.is_paused())
                .is_due(is_due);

            f.render_widget(clock, pomodoro_timer_area);
        })?;

        match rx.recv()? {
            TickContent::KeyPress(key_event) => match app.get_view() {
                AppView::Normal => match key_event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), cursor::Show, LeaveAlternateScreen)?;
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
                AppView::AnnotationPopup => match key_event.code {
                    KeyCode::Char(c) => {
                        app.append_to_interruption_annotation(c);
                    }
                    KeyCode::Backspace => {
                        app.pop_from_interruption_annotation();
                    }
                    KeyCode::Enter => {
                        app.change_view(AppView::Normal);
                    }
                    _ => {}
                },
            },
            TickContent::None => {}
        }
    }

    Ok(())
}

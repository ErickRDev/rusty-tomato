// extern crate chrono;
// use chrono::{DateTime, Utc};

use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Color},
    symbols,
    widgets::{Widget, Block},
};

#[derive(Clone)]
pub struct Clock;

impl Default for Clock {
    fn default() -> Clock {
        Clock {}
    }
}

impl Widget for Clock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let centered = Layout::default()
            .direction(Direction::Horizontal)
            .margin(5)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(area)[1];

        // TODO: devise strategy for how the graphemes should be 
        // rendered on the screen:

        let block = Block::default()
            .style(Style::default().bg(Color::Cyan));

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(5)
            .constraints(
                [
                    Constraint::Percentage(23),
                    Constraint::Percentage(23),
                    Constraint::Percentage(8),
                    Constraint::Percentage(23),
                    Constraint::Percentage(23),
                ]
                .as_ref(),
            )
            .split(centered);

        for chunk in chunks {
            let coords = get_rendering_instructions(chunk);
            for ((x, y), s) in coords {
                buf.get_mut(x, y).set_symbol(s);
            }
        }
    }
}

fn get_rendering_instructions(area: Rect) -> Vec<((u16, u16), &'static str)> {
    let mut vector: Vec<((u16, u16), &'static str)> = Vec::new();

    // println!("{:?}", area);
    // println!("first row={}; last row={}", area.y, area.height + area.y - 1);
    // println!("first col={}; last col={}", area.x, area.width + area.x - 1);

    for x in area.x..area.width + area.x {
        vector.push(((x, area.y), symbols::DOT));
        vector.push(((x, area.height + area.y - 1), symbols::DOT));
    }

    for y in area.y..area.height + area.y {
        vector.push(((area.x, y), symbols::DOT));
        vector.push(((area.width + area.x - 1, y), symbols::DOT));
    }

    vector
}

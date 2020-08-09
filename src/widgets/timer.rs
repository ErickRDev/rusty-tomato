use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::Widget,
};

#[derive(Clone)]
pub struct Timer<'a> {
    time_remaining: Option<&'a str>,
    is_due: bool,
    draw_borders: bool,
}

impl<'a> Default for Timer<'a> {
    fn default() -> Timer<'a> {
        Timer {
            time_remaining: None,
            is_due: false,
            draw_borders: false,
        }
    }
}

impl<'a> Timer<'a> {
    pub fn time_remaining(mut self, timer: &'a str) -> Timer<'a> {
        self.time_remaining = Some(timer);
        self
    }

    pub fn is_due(mut self, is_due: bool) -> Timer<'a> {
        self.is_due = is_due;
        self
    }

    pub fn draw_borders(mut self, draw_borders: bool) -> Timer<'a> {
        self.draw_borders = draw_borders;
        self
    }
}

impl<'a> Widget for Timer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let centered = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(area)[1];

        let areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(22),
                    Constraint::Percentage(2),
                    Constraint::Percentage(22),
                    Constraint::Percentage(8),
                    Constraint::Percentage(22),
                    Constraint::Percentage(2),
                    Constraint::Percentage(22),
                ]
                .as_ref(),
            )
            .split(centered);

        let areas_of_interest = vec![areas[0], areas[2], areas[3], areas[4], areas[6]];

        let graphemes = match self.time_remaining {
            Some(timer) => timer.chars(),
            None => return,
        };

        areas_of_interest
            .iter()
            .zip(graphemes)
            .for_each(|(&area, grapheme)| {
                if self.draw_borders {
                    let coords = get_rendering_instructions(area);
                    for ((x, y), s) in coords {
                        buf.get_mut(x, y).set_symbol(s);
                    }
                }

                let x = area.x + (area.width / 2);
                let y = area.y + (area.height / 2);

                let style = match self.is_due {
                    true => Style::default().fg(Color::Red),
                    false => Style::default().fg(Color::White),
                };

                buf.get_mut(x, y).set_char(grapheme).set_style(style);
            });
    }
}

fn get_rendering_instructions(area: Rect) -> Vec<((u16, u16), &'static str)> {
    let mut vector: Vec<((u16, u16), &'static str)> = Vec::new();

    for x in area.x..area.width + area.x {
        vector.push(((x, area.y), symbols::line::HORIZONTAL));
        vector.push(((x, area.height + area.y - 1), symbols::line::HORIZONTAL));
    }

    for y in area.y..area.height + area.y {
        if y == area.y {
            vector.push(((area.x, y), symbols::line::TOP_LEFT));
            vector.push(((area.width + area.x - 1, y), symbols::line::TOP_RIGHT));
        } else if y == area.height + area.y - 1 {
            vector.push(((area.x, y), symbols::line::BOTTOM_LEFT));
            vector.push(((area.width + area.x - 1, y), symbols::line::BOTTOM_RIGHT));
        } else {
            vector.push(((area.x, y), symbols::line::VERTICAL));
            vector.push(((area.width + area.x - 1, y), symbols::line::VERTICAL));
        }
    }

    vector
}

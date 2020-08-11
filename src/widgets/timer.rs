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
    draw_borders: bool,
    is_due: bool,
    is_paused: bool,
    has_been_paused_for: Option<u64>,
}

impl<'a> Default for Timer<'a> {
    fn default() -> Timer<'a> {
        Timer {
            time_remaining: None,
            draw_borders: false,
            is_due: false,
            is_paused: false,
            has_been_paused_for: None,
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

    pub fn is_paused(mut self, is_paused: bool) -> Timer<'a> {
        self.is_paused = is_paused;
        self
    }

    pub fn borders(mut self, draw_borders: bool) -> Timer<'a> {
        self.draw_borders = draw_borders;
        self
    }
}

impl<'a> Widget for Timer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {

        if self.draw_borders {
            draw_borders(&area, buf);
        }

        let timer_areas = Layout::default()
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
            .split(area);

        let graphemes_areas = vec![
            timer_areas[0],
            timer_areas[2],
            timer_areas[3],
            timer_areas[4],
            timer_areas[6],
        ];

        let graphemes = match self.time_remaining {
            Some(timer) => timer.chars(),
            None => return,
        };

        graphemes_areas
            .iter()
            .zip(graphemes)
            .for_each(|(&area, grapheme)| {
                if self.draw_borders {
                    draw_borders(&area, buf);
                }

                let x = area.x + (area.width / 2);
                let y = area.y + (area.height / 2);

                let style = match self.is_due || self.is_paused {
                    true => Style::default().fg(Color::Red),
                    false => Style::default().fg(Color::White),
                };

                buf.get_mut(x, y).set_char(grapheme).set_style(style);
            });
    }
}

/// TODO: document & extract this method into utility crate
fn draw_borders(area: &Rect, buf: &mut Buffer) {
    for x in area.x..area.width + area.x {
        buf.get_mut(x, area.y).set_symbol(symbols::line::HORIZONTAL);
        buf.get_mut(x, area.height + area.y - 1)
            .set_symbol(symbols::line::HORIZONTAL);
    }

    for y in area.y..area.height + area.y {
        if y == area.y {
            buf.get_mut(area.x, y).set_symbol(symbols::line::TOP_LEFT);
            buf.get_mut(area.width + area.x - 1, y)
                .set_symbol(symbols::line::TOP_RIGHT);
        } else if y == area.height + area.y - 1 {
            buf.get_mut(area.x, y)
                .set_symbol(symbols::line::BOTTOM_LEFT);
            buf.get_mut(area.width + area.x - 1, y)
                .set_symbol(symbols::line::BOTTOM_RIGHT);
        } else {
            buf.get_mut(area.x, y).set_symbol(symbols::line::VERTICAL);
            buf.get_mut(area.width + area.x - 1, y)
                .set_symbol(symbols::line::VERTICAL);
        }
    }
}

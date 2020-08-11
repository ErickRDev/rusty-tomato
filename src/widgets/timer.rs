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

    pub fn is_paused(mut self, is_paused: bool, has_been_paused_for: Option<u64>) -> Timer<'a> {
        self.is_paused = is_paused;
        self.has_been_paused_for = has_been_paused_for;
        self
    }

    pub fn borders(mut self, draw_borders: bool) -> Timer<'a> {
        self.draw_borders = draw_borders;
        self
    }
}

impl<'a> Widget for Timer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_area = Layout::default()
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

        let inner_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(30),
                    Constraint::Percentage(60),
                ]
                .as_ref(),
            )
            .split(outer_area);

        if self.draw_borders {
            for vertical_box in &inner_areas {
                draw_borders(vertical_box, buf);
            }
        }

        if self.is_paused {
            let pause_info_area = inner_areas[0];

            // TODO: maybe remove the unecessary 'is_paused' boolean?
            // if the `has_been_paused_for` property is Option<u64>,
            // we can use the `None` variant to verify whether the
            // timer is paused or not.
            let has_been_paused_for = self.has_been_paused_for.unwrap();

            let minutes = has_been_paused_for / 60;
            let seconds = has_been_paused_for % 60;

            // TODO: ensure there's space in buffer to write the string
            let text = format!("PAUSED {} {:02}:{:02}", symbols::DOT, minutes, seconds);
            (0..text.len())
                .map(|i| (pause_info_area.x + i as u16, pause_info_area.y))
                .zip(text.chars())
                .for_each(|((x, y), c)| {
                    &buf.get_mut(x, y)
                        .set_char(c)
                        .set_style(Style::default().fg(Color::Red));
                });
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
            .split(inner_areas[2]);

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

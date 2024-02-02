use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::Widget,
};

const GRAPHEME_UNIT_SIZE: u16 = 5;
const GRAPHEME_DRAWING_MANUAL: [[[u8; 5]; 5]; 11] = [
    /* 0 */
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    /* 1 */
    [
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    /* 2 */
    [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    /* 3 */
    [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    /* 4 */
    [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
    ],
    /* 5 */
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    /* 6 */
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    /* 7 */
    [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
    ],
    /* 8 */
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    /* 9 */
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
    ],
    /* : */
    [
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
    ],
];

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

        if self.time_remaining.is_none() {
            return;
        }

        let time_remaining_str = self.time_remaining.unwrap();

        graphemes_areas
            .iter()
            .zip(time_remaining_str.chars())
            .for_each(|(&area, c)| {
                if self.draw_borders {
                    draw_borders(&area, buf);
                }

                let style = match self.is_due || self.is_paused {
                    true => Style::default().fg(Color::Red),
                    false => Style::default().fg(Color::Gray),
                };

                let can_draw_grapheme =
                    area.height >= GRAPHEME_UNIT_SIZE && area.width >= GRAPHEME_UNIT_SIZE;

                if can_draw_grapheme {
                    let drawing_instructions = match c.to_digit(10) {
                        Some(digit) if digit <= 9 => {
                            let digit = digit as usize;
                            GRAPHEME_DRAWING_MANUAL[digit]
                        }
                        Some(_) | None => GRAPHEME_DRAWING_MANUAL[10],
                    };

                    let x0 = area.x + (area.width / 2) - (GRAPHEME_UNIT_SIZE / 2);
                    let y0 = area.y + (area.height / 2) - (GRAPHEME_UNIT_SIZE / 2);

                    for (j, row) in drawing_instructions.iter().enumerate() {
                        for (i, digit) in row.iter().enumerate() {
                            let i = i as u16;
                            let j = j as u16;

                            let symbol = match digit {
                                1 => symbols::bar::FULL,
                                _ => continue,
                            };

                            buf.get_mut(x0 + i, y0 + j)
                                .set_symbol(symbol)
                                .set_style(style);
                        }
                    }
                } else {
                    let x = area.x + (area.width / 2);
                    let y = area.y + (area.height / 2);

                    buf.get_mut(x, y).set_char(c).set_style(style);
                }
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

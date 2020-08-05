use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::Widget,
};

#[derive(Clone)]
pub struct Timer<'a> {
    current_timer: Option<&'a str>,
    draw_edges: bool,
}

impl<'a> Default for Timer<'a> {
    fn default() -> Timer<'a> {
        Timer {
            current_timer: None,
            draw_edges: false,
        }
    }
}

impl<'a> Timer<'a> {
    pub fn set_timer(mut self, timer: &'a str) -> Timer<'a> {
        self.current_timer = Some(timer);
        self
    }

    pub fn draw_edges(mut self, draw_edges: bool) -> Timer<'a> {
        self.draw_edges = draw_edges;
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

        let mut areas_of_intereset: Vec<Rect> = Vec::new();
        areas_of_intereset.push(areas[0]);
        areas_of_intereset.push(areas[2]);
        areas_of_intereset.push(areas[3]);
        areas_of_intereset.push(areas[4]);
        areas_of_intereset.push(areas[6]);

        let graphemes = match self.current_timer {
            Some(timer) => timer.chars(),
            None => return,
        };

        areas_of_intereset
            .iter()
            .zip(graphemes)
            .for_each(|(&area, grapheme)| {
                // Drawing borders
                if self.draw_edges {
                    let coords = get_rendering_instructions(area);
                    for ((x, y), s) in coords {
                        buf.get_mut(x, y).set_symbol(s);
                    }
                }

                let x = area.x + (area.width / 2);
                let y = area.y + (area.height / 2);

                buf.get_mut(x, y).set_char(grapheme);
            });
    }
}

fn get_rendering_instructions(area: Rect) -> Vec<((u16, u16), &'static str)> {
    let mut vector: Vec<((u16, u16), &'static str)> = Vec::new();

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

use std::error::Error;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{Axis, Block, Chart, Dataset},
    DefaultTerminal, Frame,
};

use crate::{chart::StarChartData, github::Star, key_code, key_code_char};

pub enum Stars {
    User(Vec<Star>),
    Repositories(Vec<(String, Vec<Star>)>),
}

pub struct App {
    data: StarChartData,
}

impl App {
    pub fn new(stars: Stars) -> Self {
        let data = StarChartData::new(stars);
        Self { data }
    }

    pub fn run(self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            #[expect(clippy::collapsible_match)]
            match event::read()? {
                Event::Key(key) => match key {
                    key_code!(KeyCode::Esc) | key_code_char!('c', Ctrl) => {
                        return Ok(());
                    }
                    _ => {}
                },
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
}

const GRAPH_COLORS: [Color; 6] = [
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Yellow,
    Color::Magenta,
    Color::Cyan,
];

impl App {
    fn draw(&self, f: &mut Frame) {
        let datasets = self
            .data
            .datasets
            .iter()
            .enumerate()
            .map(|(i, dataset)| {
                Dataset::default()
                    .name(dataset.name.clone())
                    .marker(Marker::Braille)
                    .style(Style::default().fg(GRAPH_COLORS[i % GRAPH_COLORS.len()]))
                    .data(&dataset.data)
            })
            .collect();

        let chart = Chart::new(datasets)
            .block(
                Block::bordered().title(
                    Line::from("GitHub Star History")
                        .fg(Color::Green)
                        .bold()
                        .centered(),
                ),
            )
            .x_axis(
                Axis::default()
                    .labels(self.data.x_labels.clone())
                    .bounds(self.data.x_bounds),
            )
            .y_axis(
                Axis::default()
                    .labels(self.data.y_labels.clone())
                    .bounds(self.data.y_bounds),
            );

        f.render_widget(chart, f.area());
    }
}

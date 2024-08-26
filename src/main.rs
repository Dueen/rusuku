use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    symbols::{self, border},
    text::Text,
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
    Frame,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    is_timer_running: bool,
    start_time: Option<Instant>,
    elapsed_time: Duration,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            if self.exit {
                break;
            }
            terminal.draw(|frame| {
                let area = frame.area();

                // self.render_frame(frame);
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
                    .split(area);

                render_header(frame, self, layout[0]);
                render_table(frame, self, layout[1]);
            })?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(10))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('i') => self.start_game(),
            KeyCode::Char('p') => self.stop_timer(),
            KeyCode::Char('c') => self.continue_timer(),
            _ => {}
        }
    }

    fn start_game(&mut self) {
        self.start_timer();
    }

    fn start_timer(&mut self) {
        if self.is_timer_running {
            return;
        }
        self.is_timer_running = true;
        self.start_time = Some(Instant::now());
    }

    fn stop_timer(&mut self) {
        if !self.is_timer_running {
            return;
        }
        self.is_timer_running = false;

        if let Some(start_time) = self.start_time {
            self.elapsed_time += start_time.elapsed();
            self.start_time = None;
        }
    }

    fn continue_timer(&mut self) {
        if self.is_timer_running {
            return;
        }
        self.start_time = Some(Instant::now());
        self.is_timer_running = true;
    }

    fn elapsed(&self) -> Duration {
        if let Some(start_time) = self.start_time {
            if self.is_timer_running {
                return self.elapsed_time + start_time.elapsed();
            }
        }
        self.elapsed_time
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
fn render_header(f: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" Welcome to Rusuku ".bold());

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1); 3])
        .split(area);

    let minutes = app.elapsed().as_secs() / 60;
    let seconds = app.elapsed().as_secs() % 60;
    let elapsed_time = format!("{:02}:{:02}", minutes, seconds);
    let elapsed_time = Text::from(elapsed_time.to_string().yellow().bold());

    let top_middle = Block::bordered()
        .title(title.alignment(Alignment::Center))
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_set(border::THICK);

    f.render_widget(
        Paragraph::new(elapsed_time).centered().block(top_middle),
        header_layout[1],
    );

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK),
        header_layout[0],
    );

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK),
        header_layout[2],
    );
}

fn render_table(f: &mut Frame, _: &mut App, area: Rect) {
    let vertical_layout = Layout::default()
        .constraints([Constraint::Max(18); 3])
        .direction(Direction::Horizontal)
        .flex(Flex::Center)
        .split(area);

    for (vi, vl) in vertical_layout.iter().enumerate() {
        let horizontal_layout = Layout::default()
            .constraints([Constraint::Max(18); 3])
            .direction(Direction::Vertical)
            .split(*vl);

        for (hi, hl) in horizontal_layout.iter().enumerate() {
            let border_set = match (vi, hi) {
                (0, 0) => symbols::border::Set {
                    bottom_left: symbols::line::THICK_VERTICAL_RIGHT,
                    ..symbols::border::THICK
                },
                (1, 0) => symbols::border::Set {
                    top_right: symbols::line::THICK_HORIZONTAL_DOWN,
                    top_left: symbols::line::THICK_HORIZONTAL_DOWN,
                    bottom_left: symbols::line::THICK_CROSS,
                    bottom_right: symbols::line::THICK_CROSS,
                    ..symbols::border::THICK
                },
                (2, 0) => symbols::border::Set {
                    bottom_right: symbols::line::THICK_VERTICAL_LEFT,
                    ..symbols::border::THICK
                },
                (0, 1) => symbols::border::Set {
                    bottom_left: symbols::line::THICK_VERTICAL_RIGHT,
                    ..symbols::border::THICK
                },
                (1, 1) => symbols::border::Set {
                    bottom_left: symbols::line::THICK_CROSS,
                    bottom_right: symbols::line::THICK_CROSS,
                    ..symbols::border::THICK
                },
                (2, 1) => symbols::border::Set {
                    bottom_right: symbols::line::THICK_VERTICAL_LEFT,
                    ..symbols::border::THICK
                },
                (0, 2) => symbols::border::THICK,
                (1, 2) => symbols::border::Set {
                    bottom_left: symbols::line::THICK_HORIZONTAL_UP,
                    bottom_right: symbols::line::THICK_HORIZONTAL_UP,
                    ..symbols::border::THICK
                },
                (2, 2) => symbols::border::THICK,
                _ => symbols::border::THICK,
            };

            let borders = match (vi, hi) {
                (0, 0) => Borders::LEFT | Borders::TOP | Borders::BOTTOM,
                (1, 0) => Borders::ALL,
                (2, 0) => Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
                (0, 1) => Borders::LEFT | Borders::BOTTOM,
                (1, 1) => Borders::RIGHT | Borders::LEFT | Borders::BOTTOM,
                (2, 1) => Borders::BOTTOM | Borders::RIGHT,
                (0, 2) => Borders::LEFT | Borders::BOTTOM,
                (1, 2) => Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
                (2, 2) => Borders::BOTTOM | Borders::RIGHT,
                _ => Borders::ALL,
            };

            Block::default()
                .borders(borders)
                .border_set(border_set)
                .render(*hl, f.buffer_mut());
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Welcome to Rusuku ".bold());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(area);

        let header_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1); 3])
            .split(layout[0]);

        let minutes = self.elapsed().as_secs() / 60;
        let seconds = self.elapsed().as_secs() % 60;
        let elapsed_time = format!("{:02}:{:02}", minutes, seconds);
        let elapsed_time = Text::from(elapsed_time.to_string().yellow().bold());

        let top_middle = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_set(border::THICK);

        Paragraph::new(elapsed_time)
            .centered()
            .block(top_middle)
            .render(header_layout[1], buf);

        Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .render(header_layout[0], buf);

        Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .render(header_layout[2], buf);

        Block::bordered()
            .border_set(border::THICK)
            .render(layout[1], buf);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 55, 18));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━┓ Welcome to Rusuku ┏━━━━━━━━━━━━━━━━┓",
            "┃                ┃       00:00       ┃                ┃",
            "┗━━━━━━━━━━━━━━━━┛━━━━━━━━━━━━━━━━━━━┗━━━━━━━━━━━━━━━━┛",
            "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┃                                                     ┃",
            "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
        ]);
        let title_style = Style::new().bold();
        let timer_style = Style::new().yellow().bold();
        expected.set_style(Rect::new(18, 0, 19, 1), title_style);
        expected.set_style(Rect::new(25, 1, 5, 1), timer_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}

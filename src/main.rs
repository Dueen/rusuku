use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Text,
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
    Frame,
};
use std::time::Duration;
use std::{io, time::Instant};

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
            terminal.draw(|frame| self.render_frame(frame))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            KeyCode::Char('i') => self.start_timer(),
            KeyCode::Char('p') => self.stop_timer(),
            KeyCode::Char('c') => self.continue_timer(),
            _ => {}
        }
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
        if !self.is_timer_running {
            self.start_time = Some(Instant::now());
            self.is_timer_running = true;
        }
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Welcome to Rusuku ".bold());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(area);

        let header_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
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

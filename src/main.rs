use std::io;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title}, Block, Paragraph, Widget
    },
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    fn run(&mut self, t: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            t.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) {
        if self.counter != 0 {
            self.counter -= 1;
        }
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Counter Application".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<LEFT>".blue().bold(),
            " Increment ".into(),
            "<RIGHT>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom)).border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()])]);
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}


//https://ratatui.rs/tutorials/counter-app/basic-app/
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let app_result = App::default().run(&mut terminal);
    
    ratatui::restore();
    app_result
}

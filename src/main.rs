use std::io;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::Stylize};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title}, canvas::Line, Block, Paragraph, Widget
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
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<LEFT>".blue().bold(),
            " Increment ".into(),
            "<RIGHT>".blue().bold(),
            " Quit ".into(),
            "<q>".blue().bold(),
        ]));
    }
}


//https://ratatui.rs/tutorials/counter-app/basic-app/
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let app_result = App::default().run(&mut terminal);
    
    ratatui::restore();
    app_result
}

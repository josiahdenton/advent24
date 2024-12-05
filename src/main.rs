use anyhow::Result;

use advent24::run;

fn main() -> Result<()> {
    run()
}

// TODO: will use the following to impl a term debugger / solver

//use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
//use ratatui::{
//    buffer::Buffer,
//    layout::Rect,
//    style::Stylize,
//    symbols::border,
//    text::{Line, Text},
//    widgets::{Block, Paragraph, Widget},
//    DefaultTerminal, Frame,
//};
//use std::io;
//
//fn main() -> io::Result<()> {
//    let mut terminal = ratatui::init();
//    terminal.clear()?;
//    let app_result = run(terminal);
//    ratatui::restore();
//    app_result
//}
//
//fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
//    loop {
//        terminal.draw(|frame| {
//            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
//                .white()
//                .on_blue();
//            frame.render_widget(greeting, frame.area());
//        })?;
//
//        if let event::Event::Key(key) = event::read()? {
//            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                return Ok(());
//            }
//        }
//    }
//}

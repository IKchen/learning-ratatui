use std::{io::{self,Error}, panic};
//use anyhow::Result;
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub type Frame<'a> = ratatui::Frame<'a, ratatui::backend::CrosstermBackend<std::io::Stdout>>;
pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

use crate::{app::App, event::EventHandler, ui};
pub struct Tui{
    terminal:CrosstermTerminal,
    pub event:EventHandler
}

impl Tui{
    pub fn new(terminal:CrosstermTerminal,event:EventHandler)->Self{
        Self{terminal,event}
    }
    pub fn enter(&mut self)->Result<(),std::io::Error>{
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
          Self::reset().expect("failed to reset the terminal");
          panic_hook(panic);
        }));
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }
    fn reset() -> Result<(),std::io::Error> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
      }
      pub fn exit(&mut self) -> Result<(),std::io::Error> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
      }
      pub fn draw(&mut self, app: &mut App) -> Result<(),std::io::Error> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
      }
}
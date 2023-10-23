/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

pub mod error;
//use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;
use std::{io::Error,sync::mpsc::RecvError};
use error::MyError;

fn main() -> Result<(),MyError> {
  // Create an application.
  let mut app = App::new();

  // Initialize the terminal user interface.
  let backend = CrosstermBackend::new(std::io::stdout());
  let terminal = Terminal::new(backend)?;
  let events = EventHandler::new(250);
  let mut tui = Tui::new(terminal, events);
  tui.enter()?;

  // Start the main loop.
  while !app.should_quit {
    // Render the user interface.
    tui.draw(&mut app)?;
    // Handle events.
    match tui.event.next()? {
      Event::Tick => {},
      Event::Key(key_event) => update(&mut app, key_event),
      Event::Mouse(_) => {},
      Event::Resize(_, _) => {},
    };
  }

  // Exit the user interface.
  tui.exit()?;
  Ok(())
}
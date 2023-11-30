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
/* 
async fn tokio_main()->Result<()>{
  let mut app = App::new();
  app.run().await?;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(),MyError> {
if let Err(e)=tokio_main.await{
  eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
  Err(e)
} else{Ok(())}
} */
#[tokio::main]
async fn main()->Result<(),MyError>{
  let mut app = App::new();
  app.run().await?;
  Ok(())
}
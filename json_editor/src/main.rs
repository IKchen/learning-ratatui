use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use ratatui::prelude::*;

fn main(){
    let backend=CrosstermBackend::new(io::stdout());
    let terminal=ratatui::Terminal::new(backend);
    enable_raw_mode();
    execute!(io::stdout(),EnterAlternateScreen);
    println!("nihao ");
}
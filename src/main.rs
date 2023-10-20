use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode,SetTitle, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::*,
};
use std::io::{stdout, Result};
struct App{
    counter:i32,
    should_quit:bool,
}

/* pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update; */
//pub mod app;
type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stdout >>;
fn starup()->Result<()>{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    crossterm::execute!(
        stdout(),
        SetTitle("My TUI App Title")
    )?;
   Ok(())
  
}
fn shutdown()->Result<()>{
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &mut App ,f:&mut Frame<'_>){
    f.render_widget(
        Paragraph::new(format!("Hello Ratatui! (press 'q' to quit)\n 
        counter:{}",app.counter))
            .white()
            .on_blue()
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
    f.size(),
    );

}

fn update(app:& mut App )->Result<()>{
    if event::poll(std::time::Duration::from_millis(100))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                 KeyCode::Char('q')=>app.should_quit=true,
                 KeyCode::Char('j')=>app.counter += 1,
            
                 KeyCode::Char('k')=> app.counter -= 1,
                 
                 _=>()
                } 
                
            }
        }
    }
    Ok(())
}
fn run()-> Result<()>{
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut app=App{counter:0,should_quit:false};
    loop {
        terminal.draw(|f|{ui(&mut app, f)})?;

        update(&mut app)?;
        if app.should_quit {
            break;
          }
    }
    Ok(())
}
fn main() -> Result<()> {

    starup()?;
   let result=run();
    shutdown()?;
    result?;
    Ok(())
}
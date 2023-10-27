use crossterm::terminal::{disable_raw_mode, enable_raw_mode, 
    EnterAlternateScreen, LeaveAlternateScreen};
use std::{error::Error,io};
use ratatui::prelude::*;
use crossterm::event::{self,Event,KeyCode,DisableMouseCapture, EnableMouseCapture};
mod app;
mod ui;
use crate::app::{App,
    CurrentlyEditing,CurrentScreen
};
use crate::ui::ui;
fn main()->Result<(), Box<dyn Error>> {
    let backend=CrosstermBackend::new(io::stdout());
    let terminal=ratatui::Terminal::new(backend);
    enable_raw_mode().expect("启用模式失败");
    crossterm::execute!(io::stdout(),EnterAlternateScreen,EnableMouseCapture);
    println!("nihao ");
    let app=App::new();
    let res=run_app(terminal,&app);
    disable_raw_mode()?;
    crossterm::execute!(LeaveAlternateScreen,DisableMouseCapture).expect("退出失败");
    terminal.show_cursor().expect("光标启用失败");
    if let Ok(do_print)=res{
        if do_print{
            app.print_json();
        }
    }else if let Err(err)=res{
        println!("{err:?}");
    }
    Ok(())
}
fn run_app<B:Backend>(terminal: &mut Terminal<B>,app:&mut App)-> io::Result<bool>{
    loop{
        terminal.draw(|frame|ui(app,frame));
            if let Event::Key(key)=event::read()?{
      /*   if key.kind=crossterm::event::KeyEventKind::Release{

        } */

             match app.current_screen{
                CurrentScreen::Main=>{
                    match key.code{
                        KeyCode::Char('q')|KeyCode::Esc=>{app.current_screen=CurrentScreen::Exiting}
                        KeyCode::Char('e')=>{app.current_screen=CurrentScreen::Editing}
                    //control+o output json in the terminal
                        KeyCode::Char('o')|KeyCode::Char('O')=>{
                            if key.modifiers==event::KeyModifiers::CONTROL{
                                app.current_screen=CurrentScreen::Exiting;
                            }
                        }
                        _=>Ok(())
                    }
                }
                CurrentScreen::Editing=>{
                    match key.code{
                       
                        KeyCode::Enter=>{
                            if app.currently_editing=CurrentlyEditing::Key{app.currently_editing=CurrentlyEditing::Value}
                             else {app.current_screen=CurrentScreen::Main;
                                    app.save_key_value()
                                } 
                        }
                        KeyCode::Tab=>{
                         if app.currently_editing=CurrentlyEditing::Key{app.currently_editing=CurrentlyEditing::Value}
                            else {app.currently_editing=CurrentlyEditing::Key;} 
                        }
                        KeyCode::Esc=>{app.current_screen=CurrentScreen::Main;}
                        KeyCode::Char(value)=>{
                            match app.currently_editing{
                                CurrentlyEditing::Key=>{app.key_input.push(value)}
                                CurrentlyEditing::Value=>{app.value_input.push(value)}
                                _=>Ok(())
                            }
                        }
                        //删除编辑区的字符
                        KeyCode::Backspace=>{
                            match app.currently_editing{
                                CurrentlyEditing::Key=>{app.key_input.pop()}
                                CurrentlyEditing::Value=>{app.value_input.pop()}
                                _=>Ok(())
                            }
                        }
                        _=>Ok(())
                    }
                            
                }
                CurrentScreen::Exiting=>{
                    match key.code{
                        KeyCode::Char('y')=>return true ,
                        KeyCode::Char('n')=>return false ,
                        _=>Ok(())
                    }
                }
                _=>Ok(())
            }
               
        }
    }
}

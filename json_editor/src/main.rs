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
    let mut terminal=ratatui::Terminal::new(backend)?;
    enable_raw_mode().expect("启用模式失败");
    crossterm::execute!(io::stdout(),EnterAlternateScreen,EnableMouseCapture);
    println!("nihao ");
    let mut app=App::new();
    let res=run_app(&mut terminal,&mut app);
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(),LeaveAlternateScreen,DisableMouseCapture).expect("退出失败");
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
        terminal.draw(|frame|ui(frame,app))?;
            if let Event::Key(key)=event::read()?{
                 if key.kind == event::KeyEventKind::Release {
                    // 当键盘释放时，保持状态（tab按一次后，保持选中，而不是pressing时才保持选中）
                    continue;
                } 

             match app.current_screen{
                CurrentScreen::Main=>{
                    match key.code{
                        KeyCode::Char('q')|KeyCode::Esc=>{app.current_screen=CurrentScreen::Exiting}
                        KeyCode::Char('e')=>{app.current_screen=CurrentScreen::Editing;
                            app.currently_editing = Some(CurrentlyEditing::Key);}//进入编辑屏幕时，给编辑区一个默认值
                    //control+o output json in the terminal
                        KeyCode::Char('o')|KeyCode::Char('O')=>{
                            if key.modifiers==event::KeyModifiers::CONTROL{
                                app.translate();
                            }
                        }
                        _=>()
                    }
                }
                CurrentScreen::Editing=>{
                    match key.code{
                       
                        KeyCode::Enter=>{
                            //currently_editing 是option类型, if let some 会排除掉none 情况
                            if let Some(editing)=&app.currently_editing{
                                match editing{
                                    CurrentlyEditing::Key=>{app.currently_editing=Some(CurrentlyEditing::Value)}
                                    CurrentlyEditing::Value=>{app.current_screen=CurrentScreen::Main;
                                        app.save_key_value()}
                                }
                            }
                        }
                        KeyCode::Tab=>{
                            app.toggle_editing();
                        
                        }
                        KeyCode::Esc=>{app.current_screen=CurrentScreen::Main;app.currently_editing = None;}
                        KeyCode::Char(value)=>{
                            if let Some(editing)=&app.currently_editing{
                                match editing{
                                    CurrentlyEditing::Key=>{app.key_input.push(value)}
                                    CurrentlyEditing::Value=>{app.value_input.push(value)}
                                    _=>()
                                }
                            }
                       
                        }
                        //删除编辑区的字符
                        KeyCode::Backspace=>{
                            if let Some(editing)=&app.currently_editing{
                                match editing{
                                    CurrentlyEditing::Key=>{app.key_input.pop();}
                                    CurrentlyEditing::Value=>{app.value_input.pop();}
                                    _=>()
                                }
                            }
                        
                        }
                        _=>()
                    }
                            
                }
                CurrentScreen::Exiting=>{
                    match key.code{
                        KeyCode::Char('y')=>return Ok(true),
                        KeyCode::Char('n')=>return Ok(false),
                        _=>()
                    }
                }
                _=>()
            }
               
        }
    }
}

use std::{io::{self,Error}, panic};
//use anyhow::Result;
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::{
    sync::mpsc::{self,UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub type Frame<'a> = ratatui::Frame<'a>;
//pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

use crate::{app::App, event::EventHandler, ui};
pub struct Tui{
    pub terminal:CrosstermTerminal,//终端
 //   pub event_tx:EventHandler::event_tx,//事件发送
  //  pub event_rx:EventHandler::event_rx,//事件接受
  //  pub task:JoinHandle<()>,//事件任务handle
    pub handler:EventHandler;//事件任务
    pub tick_rate:f64, //时间频率
    pub frame_rate:f64,//帧率，
    pub cancellation_token: CancellationToken,// 取消异步操作，（比如超时情况下，用户主动取消，资源管理
}

impl Tui{
    pub fn new()->Self{
        let tick_rate=4.0;
        let frame_rate=60.0;
        let terminal = ratatui::Terminal::new(CrosstermBackend::new(std::io::stde()))?;
        //let (event_tx,event_rx)=mpsc::unbounded_channel();
       // let task =tokio::spawn(async{});//new时，就初始化task，避免new之后的实例，使用时单独初始化
        let cancellation_token=CancellationToken::new();
        Self{terminal,handler,tick_rate,frame_rate,cancellation_token} 
    }
    pub fn tick_rate(&mut self, tick_rate:f64){
         self.tick_rate=tick_rate ;
    }
    pub fn frame_rate(&mut self,frame_rate:f64){
         self.frame_rate=frame_rate;
    }
    //启动应用程序
    pub fn start(){
      let tick_delay=std::time::Duration::from_secs_f64(1.0/self.tick_rate);
      let render_delay=std::time::Duration::from_secs_f64(1.0/self.frame_rate);
      self.cancel();
      self.cancellation_token=CancellationToken::new();
      let _cancellation_token = self.cancellation_token.clone();
      let handler=EventHandler::new(tick_delay,render_delay,cancellation_token);
    
    }
    //停止程序
    pub fn stop(& mut self)->Result<(),MyError>{
      self.cancel();
      let mut counter = 0;
      while !self.task.is_finished() {
        std::thread::sleep(Duration::from_millis(1));
        counter += 1;
        //等待 50 ms，50ms 之后没有完成task ，则强制终止任务 
        if counter > 50 {
          self.task.abort();
        }
        //超过100ms ，没有完成 task finished，则显示终止任务失败
        if counter > 100 {
          log::error!("Failed to abort task in 100 milliseconds for unknown reason");
          break;
        }
      }
      Ok(())
    }
    //取消异步操作
    pub fn cancel(&self) {
      self.cancellation_token.cancel();
    }
    //进入tui 模式
    pub fn enter(&mut self)->Result<(),std::io::Error>{
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        self.start();
        Ok(())
    }
/*     fn reset() -> Result<(),std::io::Error> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
      } */
      //退出tui 模式
      pub fn exit(&mut self) -> Result<(),std::io::Error> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled()? {
          //如果是raw mode 启动状态， 将缓冲区中的数据立即写入底层输出设备
          self.flush()?;
          //然后显示光标，关闭raw mode
          crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show)?;
          crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
      }
 /*      pub fn draw(&mut self, app: &mut App) -> Result<(),std::io::Error> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
      } */
      //挂起应用
      pub fn suspend(&self)->Result<(),std::io::Error>{
        //signal_hook 库在非 Windows 系统上发送 SIGTSTP 信号。SIGTSTP 信号通常用于暂停
        self.exit()?;
        #[cfg(not(windows))]
         signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
          Ok(())
      }
      //恢复应用
      pub fn resume(& self){
        self.enter()?;
        Ok(())
      }
      //获取下一个事件
      pub async fn next(&mut self)->Option<Event>{
        self.handler.next().await
      }
}
//实现引用
impl Deref for Tui {
  type Target = ratatui::Terminal<Backend<std::io::Stderr>>;

  fn deref(&self) -> &Self::Target {
    &self.terminal
  }
}
//实现可变引用
impl DerefMut for Tui {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.terminal
  }
}
//实现drop
impl Drop for Tui {
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}
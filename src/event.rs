use crossterm::event::{
    KeyEvent,MouseEvent,KeyEventKind,
    self,Event as CrosstermEvent,EventStream
};
use tokio::{
    sync::mpsc::{self,UnboundedReceiver, UnboundedSender},
    task,
};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;
use futures::{FutureExt, StreamExt};
#[derive(Clone,Debug)]
pub enum Event{
    //时间
    Tick,
    //键盘事件
    Key(KeyEvent),
    //鼠标滑动事件
    Mouse(MouseEvent),
    //重置窗口大小
    Resize(u16,u16),
    Init,//初始化
    Quit,
    Error,
    Closed,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),

}
use std::{time::{Duration,Instant},io::Error};
use futures::Stream;

#[derive(Debug)]
pub struct EventHandler{
    // 发送方
   pub event_tx:mpsc::UnboundedSender<Event>,
   // 接收方
   pub event_rx:mpsc::UnboundedReceiver<Event>,
    // 事件任务 handler 线程
    pub task: task::JoinHandle<()>,

}
/* impl EventHandler{
    //构造 event handler 实例
    pub fn new(tick_rate:f64,frame_rate:f64)->Self{
        let tick_rate=Duration::from_millis(tick_rate);
        //生成通道
        let (sender,receiver)=mpsc::channel();
        let handler={
            let sender =sender.clone();
            //生成一个事件线程，这个线程会去循环判断 事件，生成事件
            thread::spawn(move||{
                let mut last_tick=Instant::now();
                loop{
                    let timeout=tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(tick_rate);
                        //event::poll 根据timeout（等待事件有效所需要的时间），判断
                    if event::poll(timeout).expect("no events available"){
                        match event::read().expect("unable to read event"){
                            CrosstermEvent::Key(e)=>{
                                //如果 事件为键盘press 事件，则会把press的key 发送到通道中
                                if e.kind==event::KeyEventKind::Press{sender.send(Event::Key(e))}else{Ok(())}
                            },
                            CrosstermEvent::Mouse(e)=>{sender.send(Event::Mouse(e))},
                            CrosstermEvent::Resize(w,h)=>{sender.send(Event::Resize(w,h))},
                            _=>unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }
                    if last_tick.elapsed()>=tick_rate{
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick=Instant::now();
                    }
                }
            })
        };
        Self { sender, receiver, handler }
    }
    //主线程 接收下一个事件 从handler 线程中
    pub fn next(& self)->Result<Event,std::sync::mpsc::RecvError>{
        Ok(self.receiver.recv()?)
    }
    
} */

impl EventHandler{
    pub fn new()->Self{
        let (event_tx,event_rx)=tokio::sync::mpsc::unbounded_channel();
        let task = tokio::spawn(async {});
        Self{event_tx,event_rx,task}
    }
    pub fn run(& mut self,tick_delay:std::time::Duration,render_delay:std::time::Duration,cancellation_token:CancellationToken){
        let sender=self.event_tx.clone();//原始备份，避免event_tx的所有权移动到线程中
        let task=tokio::spawn(async move{
            let mut reader= crossterm::event::EventStream::new();
            //创建定时器，每个指定时间间隔触发一次
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);
            //发送初始化事件
            sender.send(Event::Init).unwrap();
            loop{
                //创建定时器，每个指定时间间隔触发一次
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                //fuse（）自动实现fusefuture ,避免重复轮询，reader 从 crossterm 事件流中读取事件
                let crossterm_event = reader.next().fuse();
                //异步执行以下事件
                tokio::select!{
                //token失效后，退出
                _ = cancellation_token.cancelled() => {
                  break;
                }
                //匹配 事件流类型
                maybe_event = crossterm_event => {
                  match maybe_event {
                    Some(Ok(evt)) => {
                      match evt {
                        CrosstermEvent::Key(key) => {
                          if key.kind == KeyEventKind::Press {
                            sender.send(Event::Key(key)).unwrap();
                          }
                        },
                        CrosstermEvent::Mouse(mouse) => {
                            sender.send(Event::Mouse(mouse)).unwrap();
                        },
                        CrosstermEvent::Resize(x, y) => {
                            sender.send(Event::Resize(x, y)).unwrap();
                        },
                        CrosstermEvent::FocusLost => {
                            sender.send(Event::FocusLost).unwrap();
                        },
                        CrosstermEvent::FocusGained => {
                            sender.send(Event::FocusGained).unwrap();
                        },
                        CrosstermEvent::Paste(s) => {
                            sender.send(Event::Paste(s)).unwrap();
                        },
                      }
                    }
                    Some(Err(_)) => {
                        sender.send(Event::Error).unwrap();
                    }
                    None => {},
                  }
                },
                _ = tick_delay => {
                    sender.send(Event::Tick).unwrap();
                },
                _ = render_delay => {
                    sender.send(Event::Render).unwrap();
                },
              }

            }

        }

        );
    }
    pub async fn next(& mut self)->Option<crate::event::Event>{
        self.event_rx.recv().await
    }
}
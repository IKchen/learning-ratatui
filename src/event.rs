use crossterm::event::{
    KeyEvent,MouseEvent,
    self,Event as CrosstermEvent
};
#[derive(Clone,Copy,Debug)]
pub enum Event{
    //时间
    Tick,
    //键盘事件
    Key(KeyEvent),
    //鼠标滑动事件
    Mouse(MouseEvent),
    //重置窗口大小
    Resize(u16,u16),
}
use std::{sync::mpsc::{self,RecvError},thread,time::{Duration,Instant},io::Error};
#[derive(Debug)]
pub struct EventHandler{
    // 发送方
    sender:mpsc::Sender<Event>,
   // 接收方
    receiver:mpsc::Receiver<Event>,
    // 事件handler 线程
    handler:thread::JoinHandle<()>,

}
impl EventHandler{
    //构造 event handler 实例
    pub fn new(tick_rate:u64)->Self{
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
    
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::{thread,time};
    #[test]
     fn test_timeout(){
   let mut last_tick=Instant::now();
   let tick_rate=time::Duration::from_millis(1000);
   loop{ 
    let timeout=tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(tick_rate);
    assert_eq!(timeout.as_millis(),1000);
    }
    }
}
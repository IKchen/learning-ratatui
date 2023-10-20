#[derive(Debug,Default)]
pub struct App{
   pub counter:i32,
    pub should_quit:bool,
}
impl App{
    //初始化
    pub fn new()->Self{
        Self::default()
    }
    pub fn tick(&self){

    }
    //结束退出
    pub fn quit(&mut self){
        self.should_quit=true;
    }
    //加
    pub fn increment_counter(&mut self){
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
          }
    }
    //减
    pub fn decrement_counter(&mut self){
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
    }   }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_increment_counter() {
      let mut app = App::default();
      app.increment_counter();
      assert_eq!(app.counter, 1);
    }
    #[test]
    fn test_decrement_counter() {
      let mut app = App::default();
      app.decrement_counter();
      assert_eq!(app.counter, 0);
    }
    #[test]
    fn test_should_quit(){
        let mut app=App::new();
        app.quit();
        assert_eq!(app.should_quit,true);
    }
}
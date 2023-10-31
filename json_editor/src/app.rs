use serde_json::Result;
use std::collections::HashMap;
//用来最终当前编辑的字段
pub enum CurrentlyEditing{
    //控制编辑区域
    Key,
    Value,
}

pub enum CurrentScreen{
    Main,//展示所有之前输入的键值对屏幕
    Editing,//展示创建键值对时的屏幕
    Exiting,//退出提示
}
pub struct App{
    pub key_input:String, //正在被编辑的key
    pub value_input:String,//正在被编辑的val
    pub pairs:HashMap<String ,String> ,//转化的键值对
    pub current_screen:CurrentScreen,//当前屏幕,会决定后续渲染什么东西
    pub currently_editing:Option<CurrentlyEditing>,//当前被编辑的kv ，未编辑任何东西就是none  

}
impl App{
    pub fn new()->Self{
        Self{
            key_input:String::new(),
            value_input:String::new(),
            pairs:HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing:None,
        }
    }
    //保存输入的kv 到键值对hashmap中
    pub fn save_key_value(&mut self){
        self.pairs.insert(self.key_input.clone(),self.value_input.clone());
       //保存后，重置这些值
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    //这段代码的目的是在两种编辑模式之间进行切换。如果当前处于 Key 编辑模式，切换后将进入 Value 编辑模式；如果当前处于 Value 编辑模式，切换后将进入 Key 编辑模式。如果尚未处于编辑模式，切换后将进入 Key 编辑模式。
    pub fn toggle_editing(&mut self){
        if let Some(edit_mode)=&self.currently_editing{
            match edit_mode{
                CurrentlyEditing::Key=>self.currently_editing=Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value=>self.currently_editing=Some(CurrentlyEditing::Key),
            };
        }else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
    //打印已存储到pairs 中的键值对
    pub fn print_json(&self)->Result<()>{
            let output=serde_json::to_string(&self.pairs)?;
            println!("{}",output);
            Ok(())
    }
    pub fn translate(&self)->Result<String>{
        let output=serde_json::to_string(&self.pairs)?;
        Ok(output)
    }

}
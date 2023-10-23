//自定义错误类型，来匹配多种错误
use std::{io::Error,sync::mpsc::RecvError};
#[derive(Debug)]
pub enum MyError{
    EventError(RecvError),
    IoError(std::io::Error),
}
//实现自定义错误的from trait 转换
impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError(error)
    }
}
impl From<RecvError> for MyError {
    fn from(error: RecvError) -> Self {
        MyError::EventError(error)
    }
}
//自定义错误类型，来匹配多种错误
use std::{io::Error,sync::mpsc::RecvError};
use tokio::sync::mpsc::error::SendError;
use crate::action::Action;
use color_eyre::eyre::ErrReport;
#[derive(Debug)]
pub enum MyError{
    EventError(RecvError),
    IoError(std::io::Error),
    SendAcitionError(tokio::sync::mpsc::error::SendError<Action>),
    ComponentError(ErrReport)
}
//实现自定义错误的from trait 转换
//输入输出错误
impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError(error)
    }
}
//接收错误
impl From<RecvError> for MyError {
    fn from(error: RecvError) -> Self {
        MyError::EventError(error)
    }
}
//异步任务发送错误
impl From<tokio::sync::mpsc::error::SendError<Action>> for MyError{
    fn from(error: SendError<Action>) -> Self {
        MyError::SendAcitionError(error)
    }
}
//组件样式错误
impl From<ErrReport> for MyError{
    fn from(error: ErrReport) -> Self {
        MyError::ComponentError(error)
    }
}
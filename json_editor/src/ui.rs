use ratatui::{widgets::{*},layout::{Constraint,Direction,Layout,Rect},
    style::{Color,Style},
    text::{Line,Span,Text},
    Frame,
    prelude::Alignment,
};
use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(f:&mut Frame,app:&App){
    let chunks=Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(5),
        Constraint::Min(2),
        Constraint::Length(3),
    ])
    .split(f.size());
    let title_block=Block::default()
    .title("this is a title")
    .title_alignment(Alignment::Left)
    .borders(Borders::All)
    .border_type(BorderType::Rounded)
    .blue()
    .split(f.size());
    f.render_widget(Paragraph::new("nihao").block(title_block),chunks[0])
}
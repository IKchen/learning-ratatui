use ratatui::{widgets::*,layout::{Constraint,Direction,Layout,Rect},
    style::{Color,Style},
    text::{Line,Span,Text},
    Frame,
    prelude::*,
    widgets::Borders,
};
use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(f:&mut Frame,app:&App){
    //main 屏幕布局
    let chunks=Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Percentage(40),
        Constraint::Percentage(40),
        Constraint::Length(3),
    ])
    .split(f.size());
        //顶部标题
    let title_block=Block::default()
    .title("这是一个标题")
    .title_alignment(Alignment::Left)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .blue();
    let title=Paragraph::new("json编辑器").block(title_block).blue();
    f.render_widget(title,chunks[0]);
        //中间屏幕，已输入的kv列表
    let mut list_item=Vec::<ListItem>::new();
        for key in app.pairs.keys(){
            list_item.push(ListItem::new(Line::from(Span::styled(
                format!("{:<25}:{}",key,app.pairs.get(key).unwrap()),
                Style::default().fg(Color::Yellow),
            ))));
        }
        let list = List::new(list_item)
        .block(Block::default().title("key value值").borders(Borders::ALL))
        .style(Style::default().fg(Color::Blue));

        f.render_widget(list, chunks[1]);
        //中间屏幕，已完成转换的kv列表
        let mut json_string=app.translate().unwrap();
        let result_title=Block::new()
        .title("json字符串转化结果")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .blue();
        let result=Paragraph::new(json_string).block(result_title).blue();
        f.render_widget(result,chunks[2]);
        //底部状态栏
    let bottom=Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(chunks[3]);
        //底部左侧的状态提示
    let current_navigation_text = vec![
        match app.current_screen{
            CurrentScreen::Main=>{Span::styled("Normal Mode",Style::default().fg(Color::Green))}
            CurrentScreen::Editing =>{Span::styled("Editing Mode",Style::default().fg(Color::Yellow))}
            CurrentScreen::Exiting=>{ Span::styled("Exiting", Style::default().fg(Color::LightRed))}
        }.to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];
    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    //右侧的按键提示
    let current_key_hint={
        match app.current_screen{
            CurrentScreen::Main=>Span::styled("(q) to quit / (e) to make new pair",Style::default().fg(Color::Red)),
            CurrentScreen::Editing=>Span::styled("(ESC) to cancel/(Tab) to switch boxes/enter to complete",Style::default().fg(Color::Red)),
            CurrentScreen::Exiting=>Span::styled(" (q) to quit / (e) to make new pair",Style::default().fg(Color::Red)),
        }
    };
    let key_notes_footer=Paragraph::new(Line::from(current_key_hint)).block(Block::default().borders(Borders::all()));
    f.render_widget(mode_footer, bottom[0]);
    f.render_widget(key_notes_footer, bottom[1]);
    //编辑文本的弹框
    if let Some(editing)=&app.currently_editing{
        let editing_screen=Block::default().title("Enter a new key-value pair").borders(Borders::NONE).style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60, 25, f.size());
        f.render_widget(editing_screen, area);
         //分成2边
        let editing_chunk=Layout::default().direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        f.render_widget(key_text, editing_chunk[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        f.render_widget(value_text, editing_chunk[1]);
    }

    //退出提示
    if let CurrentScreen::Exiting=app.current_screen{
        f.render_widget(Clear, f.size());
        let exit_block=Block::default()
        .title("Y/N")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));
        let exit_text=Text::styled("Would you like to output the buffer as json? (y/n)",
        Style::default().fg(Color::Red),);
        let exit_paragraph = Paragraph::new(exit_text)
        .block(exit_block)
        .wrap(Wrap { trim: false });
        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph,area);
    }
   
}


//中间的弹框
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
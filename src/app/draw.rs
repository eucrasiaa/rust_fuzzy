use super::FuzzyApp;
use crate::fuzzy::{SimilarityAlgorithm,FuzzyCandidate};
use std::time::Duration;
use std::io::Result;

use crossterm::{
    event::{
        self, KeyEventKind,
        DisableFocusChange, DisableMouseCapture,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode, poll
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    layout::{Constraint, Direction, Layout},
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Block, Paragraph, Widget,ListItem, List, Clear},
    DefaultTerminal, Frame,
};
impl<'a, T, A> FuzzyApp<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    /// keystroke events run every frame in run()
    pub(crate) fn draw(&mut self, frame: &mut Frame) { 
        // frame.render_widget(self, frame.area());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15), // left
                Constraint::Percentage(90), // bot
            ]).split(frame.area());

            let bot_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70), // left
                    Constraint::Percentage(30), // bot
                ]).split(chunks[1]);

                let title = Line::from(" Animal Searcher ").bold();
                let total_line = self.session.len_canidates();
                let active_line = self.session.current_results().len();
                let dropped_line = total_line-active_line;
                // let title_bottom = Line::from(format!("[total:{}] [active:{}] [dropped:{}]",)).yellow();
                let title_bottom = Line::from(vec![
                    Span::styled(format!("[total:{}]",total_line),Style::default().fg(Color::Red)),
                    Span::styled(format!("[active:{}]",active_line), Style::default().fg(Color::Green)),
                    Span::styled(format!("[dropped:{}]",dropped_line), Style::default().fg(Color::Yellow)),
                    // Span::from("!"),
                ]);

                let tui_total_line = ListItem::new(
                    Span::styled(format!("[total: {}]",total_line),Style::default().fg(Color::Red)).into_centered_line());        
                let tui_active_line = ListItem::new(
                    Span::styled(format!("[active: {}]",active_line), Style::default().fg(Color::Green)).into_centered_line());
                let tui_dropped_line = ListItem::new(
                    Span::styled(format!("[dropped: {}]",dropped_line), Style::default().fg(Color::Yellow)).into_centered_line());

                let tui_hovering_index = ListItem::new(
                    Span::styled(format!("[hover index: {}]",self.hover_index), Style::default().fg(Color::LightGreen)).into_centered_line());
                let tui_threshold_value = ListItem::new(
                    Span::styled(format!("[treshold: {}]",self.session.curr_thresh()), Style::default().fg(Color::LightRed)).into_centered_line());



                let tui_str_hovering_value = if self.hover_index==0{
                    String::from("No String Hovered")
                } else {
                    match self.session.current_results().get(self.hover_index-1){
                        Some(k) => format!("{k}"),
                        None => String::from("Error Iterating String")
                    }
                };
                let tui_hovering_value = ListItem::new(
                    Span::styled(format!("[selected: {}]",tui_str_hovering_value), Style::default().fg(Color::Cyan)).into_centered_line());

// let history_data = self.session.display_history();
//     let history_list_items: Vec<ListItem> = history_data
//     .into_iter()
//     .map(|line| {
//         ListItem::new(Line::from(vec![
//             Span::styled("[H]", Style::default().fg(Color::DarkGray)), // Optional: a little history icon
//             Span::raw(line),
//         ]))
//     })
//     .collect();

                let mut tui_db_list_vec = vec![
                    tui_total_line,tui_active_line,tui_dropped_line,
                    tui_threshold_value,
                    tui_hovering_index, tui_hovering_value, 
                ];

                // tui_db_list_vec.extend(history_list_items);
                let db_title=Line::from(" Debug Statistics ").bold();

                let tui_db_list =  List::new(tui_db_list_vec)
                    .block(
                        Block::bordered()
                        .title(db_title.centered().bg(Color::White).fg(Color::Black))
                        .border_set(border::THICK)
                    );

                // RENDER RENDER
                frame.render_widget(tui_db_list, bot_chunks[1]);

                // RENDER RENDER

                // self.list_strings.len();
                // let title = Line::from(format!(" Animal Searcher [non-cull: {}, thresh: {}]", self.session.list_strings.len(), self.matcher.threshold).bold());

                let block = Block::bordered()
                    .title(title.centered())
                    // .title_bottom(title_bottom.centered())
                    .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
                "Search: ".into(),
                self.session.current_query().to_string().yellow(),
        ])]);

        let para = Paragraph::new(counter_text)
            .centered()
            .block(block);
        frame.render_widget(para,chunks[0]);
        
        // let list_items: Vec<ListItem> = self.session.current_results()
        let list_items: Vec<ListItem> = self.session.current_results()
            .iter()
            .enumerate()
            // .enumerVate() //gives i
            .map(|(i, result)| {
    
                if i+1 == self.hover_index {
                    ListItem::new(format!("{}",result)).bg(Color::White).fg(Color::Black)
                }
                else {
                    ListItem::new(format!("{}",result))
                }
            })
        .collect();

        let event_list = List::new(list_items)
            .block(Block::bordered().title(" Results "));
        // .highlight_symbol(">> "); 
        frame.render_widget(event_list, bot_chunks[0]);
        // frame.render_widget(event_list, chunks[1]);

        if self.toggles[0] == true{
            let popup_block = Block::bordered().title("Popup");
            let centered_area = frame.area().centered(Constraint::Percentage(60), Constraint::Percentage(20));
            // clears out any background in the area before rendering the popup
            frame.render_widget(Clear, centered_area);
            let paragraph = Paragraph::new("Lorem ipsum").block(popup_block);
            frame.render_widget(paragraph, centered_area); 
        }
    }
}

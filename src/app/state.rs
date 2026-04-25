use crate::fuzzy::{SearchSession,FuzzyCandidate,SimilarityAlgorithm};

use crossterm::{
    event::{self, DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event, KeyCode, poll, KeyEventKind},
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


/// App holds a session, defined in some manor 
pub struct App<'a,T:FuzzyCandidate,S:SimilarityAlgorithm>{
    session:  SearchSession<'a,T,S>,
    exit: bool,
    keystrokes: Vec<String>, 
    toggles: Vec<i32>,
}

impl<'a,T,S> App<'a,T,S> 
where 
    T: FuzzyCandidate,
    S: SimilarityAlgorithm 
{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        execute!(io::stdout(), EnableMouseCapture, EnableFocusChange)?;
        // when run called, loop until the app's exit varible is set!
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        execute!(io::stdout(), DisableMouseCapture, DisableFocusChange)?;
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {

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
                let total_line = self.session.list_strings.len();
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
                    Span::styled(format!("[hover index: {}]",self.session.hover_index), Style::default().fg(Color::LightGreen)).into_centered_line());
                let tui_threshold_value = ListItem::new(
                    Span::styled(format!("[treshold: {}]",self.matcher.threshold), Style::default().fg(Color::LightRed)).into_centered_line());


                let tui_str_hovering_value = if self.session.hover_index==0{
                    String::from("No String Hovered")
                } else {
                    match self.session.current_results().get(self.session.hover_index-1){
                        Some(k) => format!("{k}"),
                        None => String::from("Error Iterating String")
                    }
                };
                let tui_hovering_value = ListItem::new(
                    Span::styled(format!("[selected: {}]",tui_str_hovering_value), Style::default().fg(Color::Cyan)).into_centered_line());




                let tui_db_list_vec = vec![
                    tui_total_line,tui_active_line,tui_dropped_line,
                    tui_threshold_value,
                    tui_hovering_index, tui_hovering_value
                ];
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
                self.session.current_query.to_string().yellow(),
        ])]);

        let para = Paragraph::new(counter_text)
            .centered()
            .block(block);
        frame.render_widget(para,chunks[0]);

        let list_items: Vec<ListItem> = self.session.current_results()
            .iter()
            .enumerate()
            // .enumerVate() //gives i
            .map(|(i, result)| {
                if i+1 == self.session.hover_index {
                    ListItem::new(format!("{}",result)).bg(Color::White).fg(Color::Black)
                }
                else {
                    ListItem::new(format!("{}",result))
                }
            })
        .collect();

        let event_list = List::new(list_items)
            .block(Block::bordered().title(" Results "));
        // .highlight_symbol(">> "); // Optional: if you add selection logic later
        frame.render_widget(event_list, bot_chunks[0]);
        // frame.render_widget(event_list, chunks[1]);

        if self.toggles[0] == 1{
            let popup_block = Block::bordered().title("Popup");
            let centered_area = frame.area().centered(Constraint::Percentage(60), Constraint::Percentage(20));
            // clears out any background in the area before rendering the popup
            frame.render_widget(Clear, centered_area);
            let paragraph = Paragraph::new("Lorem ipsum").block(popup_block);
            frame.render_widget(paragraph, centered_area); 
        }
    }
    fn manage_result_cursor(&mut self, arrow_dir:KeyCode){
        if self.session.hover_index == 0{ return}
        if arrow_dir==KeyCode::Up{
            if self.session.hover_index > 1 {
                self.session.hover_index-=1;
            }
        } 
        if arrow_dir==KeyCode::Down{
            if self.session.hover_index -1 < self.session.num_results{
                self.session.hover_index+=1;
            }


        }
    }
    fn handle_events(&mut self) -> io::Result<()> { 

        if poll(Duration::from_millis(16))?{
            if let Event::Key(key) = event::read()? {
                // Only handle "Press" events (avoids double-counting on Windows)
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            self.keystrokes.push(c.to_string());
                            self.session.type_char(c,&mut self.matcher); 
                            //self.session.hover_index=0;
                            if self.session.num_results > 0{
                                self.session.hover_index =1
                            }
                            else{
                                self.session.hover_index =0;
                            }

                        }
                        KeyCode::Backspace => {
                            self.keystrokes.pop();
                            self.session.backspace(); 
                            let thresh_len=self.session.thresh_len_hist.pop().unwrap_or_default();
                            self.matcher.threshold = thresh_len.0;
                            self.session.num_results = thresh_len.1;
                            if self.session.num_results > 0{
                                self.session.hover_index =1
                            }
                            else{
                                self.session.hover_index =0;
                            }
                        }
                        KeyCode::Enter => self.toggles.insert(0,self.toggles[0]^1),
                        KeyCode::Up => self.manage_result_cursor(KeyCode::Up),
                        KeyCode::Down => self.manage_result_cursor(KeyCode::Down),


                        KeyCode::Esc => {
                            self.exit = true;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
            // print_events(&mut self.keystrokes)
            //todo!()
    }
}

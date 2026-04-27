use super::DebugTui;
use crate::fuzzy::{SimilarityAlgorithm,FuzzyCandidate};

use ratatui::{
    style::{Color, Style, Stylize},
    layout::{Constraint, Direction, Layout},
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Block, Paragraph, ListItem, List, Clear},
    Frame,
};
impl<'a, T, A> DebugTui<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    pub(crate) fn draw(&mut self, frame: &mut Frame) { 
        // frame.render_widget(self, frame.area());
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // left
                Constraint::Percentage(50), // bot
            ]).split(frame.area());


        let counter_text = Text::from(vec![Line::from(vec![
                "Search: ".into(),
                self.session.current_query().to_string().yellow(),
        ])]);
                let buff_1 = Line::from(
                    Span::styled(
                        format!("{}", &self.output_buff_1),
                        Style::default().fg(Color::Red)
                    ).into_centered_line()
                );
                let buff_2 = Line::from(
                    Span::styled(
                        format!("{}", &self.output_buff_2),
                        Style::default().fg(Color::Yellow)
                    ).into_centered_line()
                );

        frame.render_widget(buff_1,chunks[0]);
        frame.render_widget(buff_2,chunks[1]);


    }
}

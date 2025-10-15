//! Terminal UI using ratatui

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

use crate::Result;
use proof_engine::ProofState;

/// Terminal UI application
pub struct Ui {
    proof_state: ProofState,
}

impl Ui {
    pub fn new(proof_state: ProofState) -> Self {
        Self { proof_state }
    }

    /// Run the TUI
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') | KeyCode::Down => {
                        // Navigate down
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        // Navigate up
                    }
                    _ => {}
                }
            }
        }

        disable_raw_mode()?;
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Title
        let title = Paragraph::new("ProveIt - Formal Verification")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Goals
        let goals: Vec<ListItem> = self
            .proof_state
            .goals()
            .enumerate()
            .map(|(i, goal)| {
                let content = format!("{}. {:?}", i + 1, goal.proposition);
                ListItem::new(content)
            })
            .collect();

        let goals_list = List::new(goals)
            .block(Block::default().borders(Borders::ALL).title("Goals"))
            .style(Style::default().fg(Color::White));
        frame.render_widget(goals_list, chunks[1]);

        // Status bar
        let status = Paragraph::new(format!(
            "Goals: {} | Complete: {} | [q]uit",
            self.proof_state.num_goals(),
            self.proof_state.completed_goals().len()
        ))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(status, chunks[2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_creation() {
        let state = ProofState::new();
        let _ui = Ui::new(state);
        assert!(true);
    }
}

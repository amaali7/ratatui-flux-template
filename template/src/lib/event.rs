use crate::Action;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub enum EventResult {
    Action(Action),
    None,
    Exit,
}

pub fn handle_events() -> Result<EventResult> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            return match key.code {
                KeyCode::Char('j') | KeyCode::Down => Ok(EventResult::Action(Action::Decrement)),
                KeyCode::Char('k') | KeyCode::Up => Ok(EventResult::Action(Action::Increment)),
                KeyCode::Char('q') | KeyCode::Esc => Ok(EventResult::Exit),
                _ => Ok(EventResult::None),
            };
        }
    }
    Ok(EventResult::None)
}

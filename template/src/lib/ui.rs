use crate::Store;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, store: &Store) {
    let counter = Paragraph::new(format!("Counter: {}", store.counter()))
        .block(Block::default().title("Flux Counter").borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(counter, frame.area());
}

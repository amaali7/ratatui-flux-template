use color_eyre::Result;
use tui_lib::{handle_events, render, Action, EventResult, Store};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut store = Store::new();

    while store.running() {
        terminal.draw(|f| render(f, &store))?;

        match handle_events()? {
            EventResult::Action(a) => store.update(a),
            EventResult::Exit => store.update(Action::Quit),
            EventResult::None => {}
        }
    }
    ratatui::restore();
    Ok(())
}

use std::io;

use tui::prelude::*;

mod app;
mod event;
mod handler;
mod ui;

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    ui::Ui,
};

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut ui = Ui::new(terminal, events);
    ui.init()?;

    while app.running {
        ui.draw(&mut app)?;

        match ui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    ui.exit()?;
    Ok(())
}

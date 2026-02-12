use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{ExecutableCommand, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::game::Game;
use crate::game::model::Language;
use crate::ui;

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> AppResult<()> {
    init_locale();
    let mut terminal = init_terminal()?;
    let run_result = event_loop(&mut terminal);
    let cleanup_result = restore_terminal(&mut terminal);

    if let Err(err) = run_result {
        if cleanup_result.is_err() {
            return Err("application crashed and terminal restore failed".into());
        }
        return Err(err);
    }

    cleanup_result?;
    Ok(())
}

fn init_locale() {
    let locale = std::env::var("RPG_LANG").unwrap_or_else(|_| "en".to_string());
    let language = Language::from_locale_tag(locale.as_str());
    rust_i18n::set_locale(language.locale_code());
}

fn event_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> AppResult<()> {
    let mut game = Game::new();
    let tick_rate = Duration::from_millis(120);

    while !game.should_quit {
        terminal.draw(|frame| ui::render(frame, &game))?;
        if event::poll(tick_rate)? {
            let evt = event::read()?;
            if let Event::Key(key) = evt
                && key.kind == KeyEventKind::Press
            {
                game.handle_key(key.code);
            }
        }
    }
    Ok(())
}

fn init_terminal() -> AppResult<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> AppResult<()> {
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

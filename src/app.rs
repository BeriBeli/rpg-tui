use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, MouseEventKind,
};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{ExecutableCommand, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;

use crate::game::Game;
use crate::ui;
use crate::ui::ScrollTarget;

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> AppResult<()> {
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

fn event_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> AppResult<()> {
    let mut game = Game::new();
    let tick_rate = Duration::from_millis(120);

    while !game.should_quit {
        terminal.draw(|frame| ui::render(frame, &game))?;
        if event::poll(tick_rate)? {
            let evt = event::read()?;
            match evt {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    game.handle_key(key.code);
                }
                Event::Mouse(mouse) => {
                    let delta = match mouse.kind {
                        MouseEventKind::ScrollUp => Some(-1),
                        MouseEventKind::ScrollDown => Some(1),
                        _ => None,
                    };
                    let Some(delta) = delta else {
                        continue;
                    };
                    let size = terminal.size()?;
                    let area = Rect::new(0, 0, size.width, size.height);
                    match ui::scroll_target_at(area, mouse.column, mouse.row) {
                        Some(ScrollTarget::Hero) => shift_scroll(&mut game.hero_scroll, delta),
                        Some(ScrollTarget::BattleLog) => shift_scroll(&mut game.log_scroll, delta),
                        Some(ScrollTarget::Controls) => {
                            shift_scroll(&mut game.controls_scroll, delta)
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn init_terminal() -> AppResult<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> AppResult<()> {
    disable_raw_mode()?;
    terminal.backend_mut().execute(DisableMouseCapture)?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn shift_scroll(scroll: &mut usize, delta: i32) {
    if delta < 0 {
        *scroll = scroll.saturating_sub(delta.unsigned_abs() as usize);
    } else if delta > 0 {
        *scroll = scroll.saturating_add(delta as usize);
    }
}

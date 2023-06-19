use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up => {
            app.arrow_up();
        }
        KeyCode::Down => {
            app.arrow_down();
        }
        KeyCode::Left => {
            app.arrow_left();
        }
        KeyCode::Right => {
            app.arrow_right();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

/// Handles the mouse events and updates the state of [`App`].
pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        // MouseEventKind::Down(_) => todo!(),
        // MouseEventKind::Up(_) => todo!(),
        // MouseEventKind::Drag(_) => todo!(),
        // MouseEventKind::Moved => todo!(),
        MouseEventKind::ScrollDown => app.text_scroll = u16::min(u16::MAX, app.text_scroll + 1),
        MouseEventKind::ScrollUp => {
            app.text_scroll = if app.text_scroll == 0 {
                0
            } else {
                app.text_scroll - 1
            }
        }
        _ => {}
    }
    Ok(())
}
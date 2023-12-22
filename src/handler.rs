use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use tui::widgets::{Paragraph, Wrap};

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
        MouseEventKind::ScrollDown => {
            if mouse_event.column <= app.text_rect.right()
                && mouse_event.column >= app.text_rect.left()
                && mouse_event.row >= app.text_rect.top()
                && mouse_event.row <= app.text_rect.bottom()
            {
                let paragraph = Paragraph::new(app.chapter_text()).wrap(Wrap { trim: false });
                let line_count = paragraph.line_count(app.text_rect.width) as u16;

                let max_scroll = if line_count < app.text_rect.height {
                    0
                } else {
                    line_count - app.text_rect.height
                };
                app.text_scroll = u16::min(max_scroll, app.text_scroll + 1)
            } else if mouse_event.column <= app.footnote_rect.right()
                && mouse_event.column >= app.footnote_rect.left()
                && mouse_event.row >= app.footnote_rect.top()
                && mouse_event.row <= app.footnote_rect.bottom()
            {
                let paragraph =
                    Paragraph::new(app.chapter_footnotes_text()).wrap(Wrap { trim: false });
                let line_count = paragraph.line_count(app.footnote_rect.width) as u16;

                let max_scroll = if line_count < app.footnote_rect.height {
                    0
                } else {
                    line_count - app.footnote_rect.height
                };
                app.footnote_scroll = u16::min(max_scroll, app.footnote_scroll + 1)
            }
        }
        MouseEventKind::ScrollUp => {
            if mouse_event.column <= app.text_rect.right()
                && mouse_event.column >= app.text_rect.left()
                && mouse_event.row >= app.text_rect.top()
                && mouse_event.row <= app.text_rect.bottom()
            {
                app.text_scroll = if app.text_scroll == 0 {
                    0
                } else {
                    app.text_scroll - 1
                }
            } else if mouse_event.column <= app.footnote_rect.right()
                && mouse_event.column >= app.footnote_rect.left()
                && mouse_event.row >= app.footnote_rect.top()
                && mouse_event.row <= app.footnote_rect.bottom()
            {
                app.footnote_scroll = if app.footnote_scroll == 0 {
                    0
                } else {
                    app.footnote_scroll - 1
                }
            }
        }
        _ => {}
    }
    Ok(())
}

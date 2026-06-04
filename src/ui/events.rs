//! Handles all event logic
use super::app::{App, Mode};
use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use std::time::Duration;

pub fn handle_events(app: &mut App) {
    if poll(Duration::from_millis(16)).expect("failed to poll events") {
        if let Ok(Event::Key(key)) = read() {
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    // Modes
                    Mode::Explorer => match &key.code {
                        KeyCode::Esc => app.running = false,
                        KeyCode::Char('j') => app.list_state.select_next(),
                        KeyCode::Char('k') => app.list_state.select_previous(),
                        KeyCode::Enter => {
                            if let Some(dir_index) = app.list_state.selected() {
                                let dir = &app.children[dir_index];
                                let path = app.current_dir.join(dir.name.clone().to_string());
                                if path.is_dir() {
                                    app.descend(path);
                                }
                            }
                        }
                        KeyCode::Backspace => app.ascend(),
                        KeyCode::Tab => app.toggle_mode(),
                        _ => {}
                    },

                    Mode::UserView => {
                        if app.selected_user == None {
                            // User view
                            match &key.code {
                                KeyCode::Esc => app.running = false,
                                KeyCode::Char('j') => app.list_state.select_next(),
                                KeyCode::Char('k') => app.list_state.select_previous(),
                                KeyCode::Enter => app.selected_user = app.list_state.selected(),
                                KeyCode::Char('S') => app.spawn_thread(),
                                KeyCode::Tab => app.toggle_mode(),
                                _ => {}
                            }
                        } else {
                            // Detailed view
                            match key.code {
                                KeyCode::Esc => app.running = false,
                                KeyCode::Backspace => {
                                    app.selected_user = None;
                                    app.list_state.select(None);
                                }
                                KeyCode::Tab => app.toggle_mode(),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

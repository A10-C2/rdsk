use std::io::{Stdout, stdout};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError};

use crate::scanner::{UserProfile, scan_users};
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::widgets::ListState;
use ratatui::{Terminal, backend::CrosstermBackend};

use super::render::{render_detailed, render_overview};

pub enum AppState {
    Idle,
    Scanning,
}

pub struct App {
    pub running: bool,
    pub state: AppState,
    pub users: Vec<UserProfile>,
    pub selected_user: Option<usize>,
    pub list_state: ListState,
    pub rx: Receiver<Vec<UserProfile>>,
    pub current_frame: usize,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let path = PathBuf::from(r"C:\Users");
        std::thread::spawn(move || {
            let users = match scan_users(&path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("scan error: {e}");
                    Vec::new()
                }
            };
            tx.send(users).ok();
        });

        App {
            running: true,
            state: AppState::Scanning,
            users: Vec::new(),
            selected_user: None,
            list_state: ListState::default(),
            rx,
            current_frame: 0,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Setup
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        // Main loop
        loop {
            if !self.running {
                break;
            } else {
                super::events::handle_events(self);

                // render
                match self.selected_user {
                    None => {
                        let users = &self.users;
                        let list_state = &mut self.list_state;
                        let state = &self.state;
                        let current_frame = &self.current_frame;
                        terminal.draw(|frame| {
                            render_overview(frame, users, list_state, state, current_frame)
                        })?;
                    }
                    Some(n) => {
                        terminal.draw(|frame| render_detailed(frame, &self.users[n]))?;
                    }
                }

                // updates
                match self.state {
                    AppState::Idle => {} // Done scanning
                    AppState::Scanning => match self.rx.try_recv() {
                        Ok(users) => {
                            self.state = AppState::Idle;
                            self.users = users;
                        }
                        Err(TryRecvError::Disconnected) => {
                            eprint!("error: {}", TryRecvError::Disconnected)
                        }
                        Err(TryRecvError::Empty) => {}
                    },
                }

                // fps count
                // dont forget that spinner is directly tied to current_frame
                self.current_frame += 1;
                if self.current_frame >= 40 {
                    self.current_frame = 0
                }
            }
        }

        // cleanup
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        println!("clean up finished...");

        Ok(())
    }

    // DON'T TOUCH
    pub fn spawn_thread(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        let path = PathBuf::from(r"C:\Users");
        std::thread::spawn(move || {
            let users = match scan_users(&path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("scan error: {e}");
                    Vec::new()
                }
            };
            tx.send(users).ok();
        });
        self.rx = rx;
        self.state = AppState::Scanning;
    }
}

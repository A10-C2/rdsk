//! Handles all app logic and thread spawning
use super::render::{render_detailed, render_overview};
use crate::scanner::{DirectoryEntry, UserProfile, scan_directory, scan_users};
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::widgets::ListState;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{Stdout, stdout};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError};

pub enum AppState {
    Idle,
    Scanning,
}

pub enum Mode {
    UserView,
    Explorer,
}

pub struct App {
    pub running: bool,
    pub mode: Mode,
    pub state: AppState,
    pub users: Vec<UserProfile>,
    pub selected_user: Option<usize>,
    pub list_state: ListState,
    pub rx: Receiver<Vec<UserProfile>>,
    pub current_frame: usize,
    pub current_dir: PathBuf,
    pub children: Vec<DirectoryEntry>,
    pub parent_dir: Option<PathBuf>,
}

impl App {
    /// Return a new instance of App and spawn a thread for `scan_users`. The initial state is set to `AppState::Scanning` until the first scan is complete.
    /// After the inital scan is completed, `self.rx` will hold the results of the scan which will allow [render_overview] to construct the users list.
    /// Until the first scan is complete menu items will not be visable.
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
            mode: Mode::Explorer,
            state: AppState::Scanning,
            users: Vec::new(),
            selected_user: None,
            list_state: ListState::default(),
            rx,
            current_frame: 0,
            current_dir: PathBuf::new(),
            children: Vec::new(),
            parent_dir: None,
        }
    }

    /// Run the app. First `enable_raw_mode()` & `EnterAlternanteScreen` then, run the main loop.
    /// After exiting the main loop, cleanup the terminal with `disable_raw_mode()` & `LeaveAlternateScreen`
    ///
    /// # Arguments
    /// * `terminal` - &mut ratatui::Terminal
    ///
    /// # Errors
    /// Returns a `std::error::Error` if anything fails.
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
                match self.mode {
                    Mode::Explorer => {
                        todo!()
                    }
                    Mode::UserView => match self.selected_user {
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
                    },
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

    /// Spawn a thread for `scan_users` and send it to be proccessed and then change the appstate to `AppState::Scanning`. Results are stored in `self.rx` after proccessing.
    /// the base path `C:\Users` is hardcoded in when the full path is constructed.
    /// # Errors
    /// If `scan_users` fails, print the scan error and return an empty vector.
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

    /// Move down a level into the specified directory
    /// # Arguments
    /// * `path` - std::path::PathBuf
    /// # Errors
    /// Returns an error if `read_dir` fails to read `path`
    pub fn descend(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.parent_dir = Some(self.current_dir.clone());
        self.current_dir = path.clone();
        let children: Vec<DirectoryEntry> = std::fs::read_dir(path)?
            .flatten()
            .map(|dir| {
                let name = dir.file_name().into_string().unwrap();
                let size = scan_directory(&dir.path());
                DirectoryEntry::new(name, size)
            })
            .collect();
        self.children = children;
        Ok(())
    }

    /// Move back into the parent directory. Then rescan and update children, current_dir, and parent_dir
    pub fn ascend(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.parent_dir {
            Some(dir) => {
                let current_dir = dir.to_path_buf();
                self.current_dir = current_dir;

                let children: Vec<DirectoryEntry> = std::fs::read_dir(dir)?
                    .flatten()
                    .map(|dir| {
                        let name = dir.file_name().into_string().unwrap();
                        let size = scan_directory(&dir.path());
                        DirectoryEntry::new(name, size)
                    })
                    .collect();
                self.children = children;

                self.parent_dir = dir.parent().map(|p| p.to_path_buf());
            }
            None => {}
        }
        Ok(())
    }
}

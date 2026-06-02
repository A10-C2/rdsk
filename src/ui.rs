use std::io::{Stdout, stdout};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

use crate::helpers::format_number;
use crate::scanner::{UserProfile, scan_users};
use crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read}, // don't forget add the key press check
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::layout::{Alignment, Margin}; // will i ever use margin, who knows
use ratatui::widgets::Paragraph;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListState, Padding},
};

pub enum AppState {
    Idle,
    Scanning,
}

pub enum View {
    Overview,
    Detailed,
}

pub struct App {
    running: bool,
    state: AppState,
    current_view: View,
    users: Vec<UserProfile>,
    selected_user: Option<usize>,
    list_state: ListState,
    rx: Receiver<Vec<UserProfile>>,
    current_frame: usize,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let path = PathBuf::from(r"C:\Users");
        // more annoying than it should have been, dont fucking touch
        // REM look into this more. already forgot how move works 4 hours later
        std::thread::spawn(move || {
            // scan
            let users = match scan_users(&path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("scan error: {e}");
                    Vec::new()
                }
            };
            //send res through tx
            tx.send(users).ok();
            // safe to touch again
        });

        App {
            running: true,
            state: AppState::Scanning,
            current_view: View::Overview,
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
        // enable raw & alt screen
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        // Main loop
        loop {
            if !self.running {
                break;
            }
            // handle events
            self.handle_global_input();
            // render
            match self.current_view {
                View::Overview => {
                    let users = &self.users;
                    let list_state = &mut self.list_state;
                    let state = &self.state;
                    let current_frame = &self.current_frame;
                    terminal.draw(|frame| {
                        render_overview(frame, users, list_state, state, current_frame)
                    })?;
                }
                View::Detailed => {
                    terminal.draw(|frame| self.render_detailed(frame))?;
                }
            }
            // updates
            match self.state {
                AppState::Idle => {} // Done
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

        // cleanup
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;

        Ok(())
    }

    fn handle_global_input(&mut self) {
        // only to escape and close program from any menu
        if poll(Duration::from_millis(16)).expect("failed to poll events") {
            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    KeyCode::Esc => self.running = false,
                    KeyCode::Char('1') => self.current_view = View::Overview,
                    KeyCode::Char('2') => self.current_view = View::Detailed,
                    _ => {}
                }
            }
        }
    }

    fn handle_input(&mut self) {
        // real input handler
        // update lol nevermind
        todo!();
    }

    // probably dont need this shit
    fn render_detailed(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(30), Constraint::Percentage(70)],
        );
        let [left, right] = layout.areas(frame.area());

        let blockLeft = Block::default().borders(Borders::ALL);
        let blockRight = Block::default().borders(Borders::ALL);
        let border = Block::bordered().padding(Padding::proportional(1));

        frame.render_widget(blockLeft, left);
        frame.render_widget(blockRight, right);
    }
}

fn render_overview(
    frame: &mut Frame,
    users: &Vec<UserProfile>,
    list_state: &mut ListState,
    state: &AppState,
    current_frame: &usize,
) {
    // spinner stuff
    let spin = [r"/", r"-", r"\", r"|"];
    let spin_index = current_frame / 10 % spin.len();

    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(10), Constraint::Percentage(90)],
    );

    let [status, body] = layout.areas(frame.area());

    let [_top, middle, _bottom] = Layout::new(
        Direction::Vertical,
        [
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ],
    )
    .areas(status);

    // before investing in $rope, look here
    let items: Vec<String> = users
        .iter()
        .map(|u| {
            let username = u.username.to_string();
            let total_size = format_number(u.total_size);
            format!("{:<10} {:<4}", username, total_size)
        })
        .collect();

    let app_state = match state {
        AppState::Idle => format!(" Idle "),
        AppState::Scanning => {
            format!(" Scanning  {:^3}", spin[spin_index])
        }
    };

    let status_line = Paragraph::new(app_state)
        .alignment(Alignment::Center)
        .centered();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" List ")
                .padding(Padding::symmetric(2, 1)),
        )
        .direction(ratatui::widgets::ListDirection::TopToBottom)
        .highlight_symbol(">")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

    frame.render_widget(status_line, middle);
    frame.render_stateful_widget(list, body, list_state);
}

/*
TODO:
    add color
    font size?
    term size?
    probs can fit everything in one split window
 */

use std::io::{Stdout, stdout};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

use crate::helpers::format_number;
use crate::scanner::{UserProfile, scan_users};
use crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, KeyEventKind, poll, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::layout::Alignment;
use ratatui::widgets::Paragraph;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListState, Padding},
};

pub enum AppState {
    Idle,
    Scanning,
}

pub struct App {
    running: bool,
    state: AppState,
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
        // enable raw & alt screen
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        // Main loop
        loop {
            if !self.running {
                break;
            }

            // handle events
            self.handle_events();

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
        println!("clean up finished...");

        Ok(())
    }

    fn handle_events(&mut self) {
        if poll(Duration::from_millis(16)).expect("failed to poll events") {
            if let Ok(Event::Key(key)) = read() {
                if key.kind == KeyEventKind::Press {
                    if self.selected_user == None {
                        // User view
                        match &key.code {
                            KeyCode::Esc => self.running = false,
                            KeyCode::Char('j') => self.list_state.select_next(),
                            KeyCode::Char('k') => self.list_state.select_previous(),
                            KeyCode::Enter => self.selected_user = self.list_state.selected(),
                            KeyCode::Char('S') => self.spawn_thread(),
                            _ => {}
                        }
                    } else {
                        // Detailed view
                        match key.code {
                            KeyCode::Esc => self.running = false,
                            KeyCode::Backspace => {
                                self.selected_user = None;
                                self.list_state.select(None);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn spawn_thread(&mut self) {
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

    // layout shit
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ],
    );

    let [status, body, footer] = layout.areas(frame.area());

    // just the status line
    let [_top, middle, _bottom] = Layout::new(
        Direction::Vertical,
        [
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ],
    )
    .areas(status);

    // List shit
    // before investing in $rope, look here
    let items: Vec<String> = users
        .iter()
        .map(|u| {
            let username = u.username.to_string();
            let total_size = format_number(u.total_size);
            format!("{:<20}\t{:<4}", username, total_size) // here retard
        })
        .collect();

    let app_state = match state {
        AppState::Idle => format!(" Idle "),
        AppState::Scanning => format!("Scanning  {:^3}", spin[spin_index]),
    };

    // Widgets
    let status_line = Paragraph::new(app_state)
        .alignment(Alignment::Center)
        .centered()
        .light_cyan();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" User Overview ")
                .title_alignment(Alignment::Center)
                .style(Style::new().yellow())
                .padding(Padding::symmetric(2, 1)),
        )
        .light_cyan()
        .scroll_padding(1)
        .direction(ratatui::widgets::ListDirection::TopToBottom)
        .highlight_symbol("-> ")
        .highlight_style(Style::new().reversed())
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

    // Controls
    let mut controls: Vec<Line> = Vec::new();
    controls.push(Line::from("<j> ↓↑ <k>"));
    controls.push(Line::from("<Enter>"));
    controls.push(Line::from("<Esc> Exit"));
    controls.push(Line::from("<S> Start Scan"));

    let control_block = Paragraph::new(controls)
        .block(Block::default().borders(Borders::ALL).title(" Controls "))
        .centered()
        .yellow();

    frame.render_widget(status_line, middle);
    frame.render_stateful_widget(list, body, list_state);
    frame.render_widget(control_block, footer);
}

fn render_detailed(frame: &mut Frame, user: &UserProfile) {
    let username = user.username.clone();
    let total = format_number(user.total_size);
    let appdata_local = format_number(user.appdata_local);
    let appdata_roaming = format_number(user.appdata_roaming);
    let appdata_local_temp = format_number(user.appdata_local_temp);
    let desktop = format_number(user.desktop);
    let documents = format_number(user.documents);
    let downloads = format_number(user.downloads);
    let teams_cache = format_number(user.teams_cache);
    let one_drive = format_number(user.onedrive);
    let other = format_number(user.other);

    let text = format!(
        "Username: {:^10}\n
    Total Size: {:<5}\n
    AppData Local: {:<5}\n
    AppData Roaming: {:<5}\n
    AppData Local Temp: {:<5}\n
    Desktop: {:<5}\n
    Documents: {:<5}\n
    Downloads: {:<5}\n
    Teams Cache: {:<5}\n
    Onedrive: {:<5}\n
    Other: {:<5}",
        username,
        total,
        appdata_local,
        appdata_roaming,
        appdata_local_temp,
        desktop,
        documents,
        downloads,
        teams_cache,
        one_drive,
        other
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Detailed View ")
        .title_bottom(" <Backspace> ")
        .title_alignment(Alignment::Center)
        .padding(Padding::symmetric(1, 2))
        .style(Style::new().light_cyan());

    let body = Paragraph::new(text).block(block);

    frame.render_widget(body, frame.area())
}

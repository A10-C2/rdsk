use super::app::AppState;
use crate::helpers::format_number;
use crate::scanner::UserProfile;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListState, Padding, Paragraph},
};

pub fn render_overview(
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
            Constraint::Fill(1),
            Constraint::Length(6),
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
            format!("{:<30}\t\t{:<4}", username, total_size) // here retard
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

pub fn render_detailed(frame: &mut Frame, user: &UserProfile) {
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
        .style(Style::new().yellow());

    let body = Paragraph::new(text).block(block);

    frame.render_widget(body, frame.area())
}

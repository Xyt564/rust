use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Stdout};

use serde::{Deserialize, Serialize};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph},
    Frame, Terminal,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    description: String,
    status: Status,
}

const FILE_PATH: &str = "tasks.json";

// Palette
const ACCENT: Color = Color::Rgb(97, 175, 239); // soft blue
const ACCENT_ALT: Color = Color::Rgb(198, 120, 221); // magenta, used for the popup
const GOOD: Color = Color::Rgb(152, 195, 121); // green
const WARN: Color = Color::Rgb(229, 192, 123); // yellow
const BAD: Color = Color::Rgb(224, 108, 117); // red
const MUTED: Color = Color::Rgb(120, 128, 140); // gray
const SELECTED_BG: Color = Color::Rgb(44, 51, 68);

enum Mode {
    Normal,
    Adding,
}

#[derive(Clone, Copy)]
enum StatusKind {
    Info,
    Success,
    Error,
}

impl StatusKind {
    fn color(self) -> Color {
        match self {
            StatusKind::Info => MUTED,
            StatusKind::Success => GOOD,
            StatusKind::Error => BAD,
        }
    }
}

struct App {
    tasks: Vec<Task>,
    list_state: ListState,
    mode: Mode,
    input: String,
    status_message: String,
    status_kind: StatusKind,
}

impl App {
    fn new(tasks: Vec<Task>) -> App {
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }
        App {
            tasks,
            list_state,
            mode: Mode::Normal,
            input: String::new(),
            status_message: "Welcome! Press 'a' to add a task.".to_string(),
            status_kind: StatusKind::Info,
        }
    }

    fn set_status(&mut self, msg: impl Into<String>, kind: StatusKind) {
        self.status_message = msg.into();
        self.status_kind = kind;
    }

    fn next(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.tasks.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(0) | None => self.tasks.len() - 1,
            Some(i) => i - 1,
        };
        self.list_state.select(Some(i));
    }

    fn toggle_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(task) = self.tasks.get_mut(i) {
                task.status = match task.status {
                    Status::Pending => Status::Done,
                    Status::Done => Status::Pending,
                };
                save_tasks(&self.tasks);
                self.set_status("Task status toggled.", StatusKind::Success);
            }
        }
    }

    fn delete_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.tasks.len() {
                self.tasks.remove(i);
                save_tasks(&self.tasks);
                if self.tasks.is_empty() {
                    self.list_state.select(None);
                } else if i >= self.tasks.len() {
                    self.list_state.select(Some(self.tasks.len() - 1));
                }
                self.set_status("Task deleted.", StatusKind::Success);
            }
        } else {
            self.set_status("No task selected to delete.", StatusKind::Error);
        }
    }

    // Returns true if a task was actually added.
    fn add_task(&mut self) -> bool {
        let desc = self.input.trim().to_string();
        self.input.clear();
        if desc.is_empty() {
            self.set_status("Task description can't be empty.", StatusKind::Error);
            return false;
        }
        self.tasks.push(Task {
            description: desc,
            status: Status::Pending,
        });
        save_tasks(&self.tasks);
        self.list_state.select(Some(self.tasks.len() - 1));
        self.set_status("Task added.", StatusKind::Success);
        true
    }

    fn done_count(&self) -> usize {
        self.tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Done))
            .count()
    }
}

fn main() -> io::Result<()> {
    let tasks = load_tasks();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(tasks);
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('a') => {
                        app.mode = Mode::Adding;
                        app.input.clear();
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter | KeyCode::Char(' ') => app.toggle_selected(),
                    KeyCode::Char('d') => app.delete_selected(),
                    _ => {}
                },
                Mode::Adding => match key.code {
                    KeyCode::Enter => {
                        if app.add_task() {
                            app.mode = Mode::Normal;
                        }
                    }
                    KeyCode::Esc => {
                        app.input.clear();
                        app.mode = Mode::Normal;
                        app.set_status("Cancelled.", StatusKind::Info);
                    }
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_title(f, app, chunks[0]);
    draw_task_list(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);

    if let Mode::Adding = app.mode {
        draw_add_popup(f, app);
    }
}

fn draw_title(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT));

    let stats = format!("{} done / {} total", app.done_count(), app.tasks.len());

    let lines = vec![
        Line::from(Span::styled(
            "TUI TODO",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(stats, Style::default().fg(MUTED))),
    ];

    let title = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(block);
    f.render_widget(title, area);
}

fn draw_task_list(f: &mut Frame, app: &App, area: Rect) {
    let border_color = match app.mode {
        Mode::Normal => ACCENT,
        Mode::Adding => MUTED,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(
            format!(" Tasks ({}) ", app.tasks.len()),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ));

    if app.tasks.is_empty() {
        let empty = Paragraph::new(Line::from(Span::styled(
            "No tasks yet — press 'a' to add one.",
            Style::default()
                .fg(MUTED)
                .add_modifier(Modifier::ITALIC),
        )))
        .alignment(Alignment::Center)
        .block(block);
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let (checkbox, text_style) = match task.status {
                Status::Done => (
                    Span::styled("✓ ", Style::default().fg(GOOD).add_modifier(Modifier::BOLD)),
                    Style::default().fg(MUTED).add_modifier(Modifier::CROSSED_OUT),
                ),
                Status::Pending => (
                    Span::styled("○ ", Style::default().fg(WARN)),
                    Style::default().fg(Color::White),
                ),
            };
            ListItem::new(Line::from(vec![
                checkbox,
                Span::styled(task.description.clone(), text_style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(SELECTED_BG)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("» ");

    f.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(MUTED));

    let key_style = Style::default().fg(ACCENT).add_modifier(Modifier::BOLD);
    let desc_style = Style::default().fg(MUTED);
    let sep_style = Style::default().fg(MUTED);

    let line = match app.mode {
        Mode::Normal => Line::from(vec![
            Span::styled("a", key_style),
            Span::styled(" add  ", desc_style),
            Span::styled("•  ", sep_style),
            Span::styled("j/k ↑/↓", key_style),
            Span::styled(" move  ", desc_style),
            Span::styled("•  ", sep_style),
            Span::styled("space/enter", key_style),
            Span::styled(" toggle  ", desc_style),
            Span::styled("•  ", sep_style),
            Span::styled("d", key_style),
            Span::styled(" delete  ", desc_style),
            Span::styled("•  ", sep_style),
            Span::styled("q", key_style),
            Span::styled(" quit    ", desc_style),
            Span::styled(
                &app.status_message,
                Style::default()
                    .fg(app.status_kind.color())
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Mode::Adding => Line::from(vec![
            Span::styled("Enter", key_style),
            Span::styled(" confirm  ", desc_style),
            Span::styled("•  ", sep_style),
            Span::styled("Esc", key_style),
            Span::styled(" cancel", desc_style),
        ]),
    };

    let footer = Paragraph::new(line).block(block);
    f.render_widget(footer, area);
}

fn draw_add_popup(f: &mut Frame, app: &App) {
    let popup_area = centered_rect(50, 20, f.area());
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT_ALT))
        .title(Span::styled(
            " New Task ",
            Style::default().fg(ACCENT_ALT).add_modifier(Modifier::BOLD),
        ))
        .padding(Padding::uniform(1));

    let cursor = Span::styled("▏", Style::default().fg(ACCENT_ALT));
    let content = Line::from(vec![
        Span::styled(app.input.as_str(), Style::default().fg(Color::White)),
        cursor,
    ]);

    let input_box = Paragraph::new(content).block(block);
    f.render_widget(input_box, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn save_tasks(tasks: &Vec<Task>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Unable to open file");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &tasks).expect("Failed to write JSON");
}

fn load_tasks() -> Vec<Task> {
    let file = File::open(FILE_PATH);
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        let tasks: Result<Vec<Task>, _> = serde_json::from_reader(reader);
        match tasks {
            Ok(t) => t,
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

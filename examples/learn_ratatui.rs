use std::{io::Stdout, ops::ControlFlow, time::Duration};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend as Backend,
    prelude::*,
    style::Stylize,
    terminal,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Terminal = terminal::Terminal<Backend<Stdout>>;

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    let res = run(&mut terminal);

    restore_terminal(terminal)?;

    if let Err(err) = res {
        eprintln!("main->run() function occurs error: {err}");
    }

    Ok(())
}

fn run(terminal: &mut Terminal) -> Result<()> {
    loop {
        terminal.draw(ui)?;

        if handle_events()?.is_break() {
            return Ok(());
        }
    }
}

fn calculate_layout(area: Rect) -> (Rect, Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Max(3)])
        .split(area);
    let msg_layout = layout[0];
    let input_layout = layout[1];
    (msg_layout, input_layout)
}

fn ui(frame: &mut Frame) {
    let area = frame.size();
    let (msg_window, input_window) = calculate_layout(area);

    render_msg_window(frame, "msg window", BorderType::Double, msg_window);
    render_input_window(frame, "input window", BorderType::Plain, input_window);
}

fn render_msg_window(frame: &mut Frame, text: &str, border_type: BorderType, layout: Rect) {
    let paragraph = Paragraph::new(text.blue()).wrap(Wrap { trim: true });
    let block = Block::new().borders(Borders::ALL).border_type(border_type);
    frame.render_widget(paragraph.block(block), layout);
}

fn render_input_window(frame: &mut Frame, text: &str, border_type: BorderType, layout: Rect) {
    let paragraph = Paragraph::new(text.red()).wrap(Wrap { trim: true });
    let block = Block::new().borders(Borders::ALL).border_type(border_type);
    frame.render_widget(paragraph.block(block), layout);
}

fn handle_events() -> Result<ControlFlow<()>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(ControlFlow::Break(()));
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

fn setup_terminal() -> Result<Terminal> {
    let mut stdout = std::io::stdout();
    // enter raw-mode and Alternative-screen
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = Backend::new(stdout);
    let terminal = terminal::Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal) -> Result<()> {
    // leave raw-mode and Alternative-screen
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}

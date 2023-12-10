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
// use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, ()>;
type Terminal = terminal::Terminal<Backend<Stdout>>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    run(&mut terminal)?;

    restore_terminal(terminal)?;
    Ok(())
}

fn run(terminal: &mut Terminal) -> Result<()> {
    loop {
        terminal.draw(ui).unwrap();

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
    if event::poll(Duration::from_millis(100))
        .map_err(|err| eprintln!("event poll failed: {err}"))?
    {
        if let Event::Key(key) =
            event::read().map_err(|err| eprintln!("event read failed: {err}"))?
        {
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
    crossterm::terminal::enable_raw_mode().map_err(|err| {
        eprintln!("ERROR: enable raw mode failed: {err}");
    })?;
    crossterm::execute!(stdout, EnterAlternateScreen, cursor::Hide).map_err(|err| {
        eprintln!("ERROR: enter alternative screen failed: {err}");
    })?;
    let backend = Backend::new(stdout);
    let terminal = terminal::Terminal::new(backend).map_err(|err| {
        eprintln!("ERROR: create terminal failed: {err}");
    })?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal) -> Result<()> {
    // leave raw-mode and Alternative-screen
    let _ = crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)
        .map_err(|err| {
            eprintln!("ERROR: leave alternative screen failed: {err}");
        });
    crossterm::terminal::disable_raw_mode().map_err(|err| {
        eprintln!("ERROR: disable raw mode failed: {err}");
    })?;
    Ok(())
}

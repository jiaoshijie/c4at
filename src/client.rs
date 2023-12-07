use crossterm::{
    cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend as Beckend, widgets::Paragraph};
use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, ()>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut terminal =
        ratatui::terminal::Terminal::new(Beckend::new(std::io::stdout())).map_err(|err| {
            eprintln!("ERROR: create terminal failed: {err}");
        })?;

    // enter raw-mode and Alternative-screen
    crossterm::terminal::enable_raw_mode().map_err(|err| {
        eprintln!("ERROR: enable raw mode failed: {err}");
    })?;
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide).map_err(|err| {
        eprintln!("ERROR: enter alternative screen failed: {err}");
    })?;

    terminal
        .draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new("Hello World!"), area);
            frame.set_cursor(0, 0);
        })
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    // leave raw-mode and Alternative-screen
    let _ =
        crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show).map_err(|err| {
            eprintln!("ERROR: leave alternative screen failed: {err}");
        });
    crossterm::terminal::disable_raw_mode().map_err(|err| {
        eprintln!("ERROR: disable raw mode failed: {err}");
    })?;
    Ok(())
}

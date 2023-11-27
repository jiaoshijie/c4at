use crossterm::{
    cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend as Beckend, widgets::Paragraph};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut terminal = ratatui::terminal::Terminal::new(Beckend::new(std::io::stdout())).unwrap();

    // enter raw-mode and Alternative-screen
    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

    terminal
        .draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new("Hello World!"), area);
            frame.set_cursor(0, 0);
        })
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    // leave raw-mode and Alternative-screen
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}

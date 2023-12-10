use ratatui::layout::Constraint;

fn main() {
    println!("{}", Constraint::Ratio(2, 10).apply(10));
}

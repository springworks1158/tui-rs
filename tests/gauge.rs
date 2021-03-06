use tui::backend::TestBackend;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Gauge};
use tui::Terminal;

#[test]
fn gauge_render() {
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let gauge = Gauge::default()
                .block(Block::default().title("Percentage").borders(Borders::ALL))
                .percent(43);
            f.render_widget(gauge, chunks[0]);
            let gauge = Gauge::default()
                .block(Block::default().title("Ratio").borders(Borders::ALL))
                .ratio(0.211_313_934_313_1);
            f.render_widget(gauge, chunks[1]);
        })
        .unwrap();
    let expected = Buffer::with_lines(vec![
        "                                        ",
        "                                        ",
        "  ┌Percentage────────────────────────┐  ",
        "  │               43%                │  ",
        "  └──────────────────────────────────┘  ",
        "  ┌Ratio─────────────────────────────┐  ",
        "  │               21%                │  ",
        "  └──────────────────────────────────┘  ",
        "                                        ",
        "                                        ",
    ]);
    assert_eq!(&expected, terminal.backend().buffer());
}

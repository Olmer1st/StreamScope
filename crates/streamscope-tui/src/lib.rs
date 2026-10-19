//! Terminal presentation; capture runs independently in a bounded shared model.
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table, TableState,
    },
    Frame, Terminal,
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};
use streamscope_core::{Shared, State, StreamEndReason};
use tokio::sync::watch;

struct Restore;
impl Drop for Restore {
    fn drop(&mut self) {
        restore();
    }
}
fn restore() {
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
}

/// Render bounded state at any terminal size. Selection is an index in the retained ring.
pub fn draw(frame: &mut Frame, state: &State, selected: usize, frozen: bool, elapsed: Duration) {
    if frame.area().width < 60 || frame.area().height < 15 {
        frame.render_widget(
            Paragraph::new("StreamScope: enlarge terminal to at least 60x15; q quits"),
            frame.area(),
        );
        return;
    }
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(4),
            Constraint::Length(5),
            Constraint::Length(2),
        ])
        .split(frame.area());
    let m = state.metrics();
    frame.render_widget(Paragraph::new(format!(
        "StreamScope  {}  {:?}  {:.1}s\nHTTP {:?}  events {}  bytes {}  evicted {}\nClient-observed: first byte {:?}ms  first event {:?}ms  max gap {:?}ms",
        if frozen { "FROZEN" } else { "LIVE" }, state.end, elapsed.as_secs_f64(), state.http_status,
        m.event_count, m.total_bytes, m.evicted_events, m.first_byte_ms, m.first_event_ms, m.max_observed_gap_ms
    )).block(Block::default().borders(Borders::ALL)), areas[0]);
    let rows = state.events.iter().map(|event| {
        let gap = event.gap_ms.unwrap_or(0);
        let bars = "▇".repeat((gap / 100).min(28) as usize);
        Row::new(vec![
            Cell::from(event.ordinal.to_string()),
            Cell::from(format!("{}ms", event.elapsed_ms)),
            Cell::from(format!("{gap}ms")),
            Cell::from(event.event_type.clone()),
            Cell::from(event.frame_bytes.to_string()),
            Cell::from(bars),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(7),
            Constraint::Min(4),
        ],
    )
    .header(Row::new([
        "#",
        "elapsed",
        "gap",
        "event",
        "bytes",
        "gap timeline",
    ]))
    .block(
        Block::default()
            .title("Events (bounded to 256)")
            .borders(Borders::ALL),
    )
    .row_highlight_style(Style::default().bg(Color::DarkGray));
    let mut table_state = TableState::default().with_selected(Some(selected));
    frame.render_stateful_widget(table, areas[1], &mut table_state);
    let mut scrollbar = ScrollbarState::new(state.events.len()).position(selected);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight),
        areas[1],
        &mut scrollbar,
    );
    let detail = state
        .events
        .get(selected)
        .map(|event| {
            format!(
                "Event {} • {} bytes • preview ≤512 characters\n{}",
                event.ordinal, event.frame_bytes, event.data
            )
        })
        .unwrap_or_else(|| "Waiting for complete events".into());
    frame.render_widget(
        Paragraph::new(detail).block(
            Block::default()
                .title("Live payload preview (never saved)")
                .borders(Borders::ALL),
        ),
        areas[2],
    );
    let finding = state
        .findings
        .last()
        .map(|f| f.code.as_str())
        .unwrap_or("Timings are client observations, not server execution timings");
    frame.render_widget(
        Paragraph::new(format!(
            "j/k ↑/↓ navigate • space freeze view • q quit • Ctrl-C interrupt\n{finding}"
        )),
        areas[3],
    );
}

/// Enter an alternate screen and restore on normal return, errors, or panic.
/// Freeze snapshots only the presentation; the capture task continues updating its ring.
pub async fn run(state: Shared, cancel: watch::Sender<bool>) -> io::Result<u8> {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        previous_hook(info);
    }));
    let _restore = Restore;
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let started = Instant::now();
    let mut selected = 0usize;
    let mut frozen: Option<State> = None;
    let mut tick = tokio::time::interval(Duration::from_millis(50));
    loop {
        tick.tick().await;
        let live = state.lock().unwrap().clone();
        let mut snapshot = frozen.clone().unwrap_or_else(|| live.clone());
        // Keep resource-eviction evidence live even while the viewport is frozen.
        snapshot.metrics.evicted_events = live.metrics.evicted_events;
        selected = selected.min(snapshot.events.len().saturating_sub(1));
        terminal.draw(|frame| {
            draw(
                frame,
                &snapshot,
                selected,
                frozen.is_some(),
                started.elapsed(),
            )
        })?;
        if event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => {
                        let _ = cancel.send(true);
                        return Ok(0);
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        let _ = cancel.send(true);
                        return Ok(130);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        selected = (selected + 1).min(snapshot.events.len().saturating_sub(1))
                    }
                    KeyCode::Up | KeyCode::Char('k') => selected = selected.saturating_sub(1),
                    KeyCode::Char(' ') => {
                        frozen = if frozen.is_some() {
                            None
                        } else {
                            Some(snapshot)
                        }
                    }
                    _ => {}
                }
            }
        }
        if state.lock().unwrap().end == StreamEndReason::Cancelled {
            return Ok(130);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    #[test]
    fn populated_dashboard_matches_golden_snapshots() {
        use streamscope_core::sse::Event;
        let mut state = State::default();
        state.http_status = Some(200);
        state.valid_content_type = true;
        state.metrics.first_byte_ms = Some(10);
        for (index, elapsed) in [150, 300, 4400, 4550].into_iter().enumerate() {
            state.event(
                Event {
                    event_type: if index == 3 {
                        "done".into()
                    } else {
                        "delta".into()
                    },
                    id: (index + 1).to_string(),
                    data: format!("synthetic {}", index + 1),
                    frame_bytes: 40,
                },
                elapsed,
                Some(1500),
                0,
            );
        }
        state.end = StreamEndReason::Eof;
        for (width, height) in [(80, 24), (120, 40)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            terminal
                .draw(|f| draw(f, &state, 2, false, Duration::from_secs(5)))
                .unwrap();
            let buffer = terminal.backend().buffer();
            let mut actual = String::new();
            for row in buffer.content().chunks(width as usize) {
                let line: String = row.iter().map(|c| c.symbol()).collect();
                actual.push_str(line.trim_end());
                actual.push('\n');
            }
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("tests/snapshots/dashboard-{width}x{height}.txt"));
            if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(&path, &actual).unwrap();
            }
            assert_eq!(
                actual,
                std::fs::read_to_string(&path).unwrap(),
                "snapshot {}",
                path.display()
            );
            assert!(actual.contains("GAP_EXCEEDED"));
            assert!(actual.contains("synthetic 3"));
        }
    }

    #[test]
    fn responsive_render_and_terminal_controls_are_safe() {
        for (width, height) in [(80, 24), (120, 40), (20, 5)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            terminal
                .draw(|f| draw(f, &State::default(), 0, false, Duration::ZERO))
                .unwrap();
            let buffer = terminal.backend().buffer();
            let text: String = buffer.content().iter().map(|c| c.symbol()).collect();
            assert!(text.contains("StreamScope"));
        }
    }
}

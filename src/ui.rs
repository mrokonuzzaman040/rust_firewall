// src/ui.rs

use crate::TrafficData;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::sync::MutexGuard;
use std::time::Duration;

pub fn run_tui(traffic_data: TrafficData) {
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

    let mut filter = String::new();
    let mut filtering = false;

    loop {
        terminal
            .draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(frame.size());

                let traffic_map: MutexGuard<HashMap<String, (usize, String)>> =
                    traffic_data.lock().unwrap();

                // Convert to Vec
                let mut data_vec: Vec<(usize, String, String)> = traffic_map
                    .iter()
                    .map(|(conn, (bytes, proto))| (*bytes, conn.clone(), proto.clone()))
                    .collect();

                // Sort by bytes descending
                data_vec.sort_by(|a, b| b.0.cmp(&a.0));

                // Filter if needed
                if filtering && !filter.is_empty() {
                    let re = Regex::new(&filter).unwrap();
                    data_vec.retain(|(_, connection, _)| re.is_match(connection));
                }

                // Convert to rows
                let rows: Vec<Row> = data_vec
                    .iter()
                    .map(|(bytes, connection, proto)| {
                        let color = match proto.as_str() {
                            "TCP" => Color::Green,
                            "UDP" => Color::Blue,
                            "ICMP" | "ICMPv6" => Color::Red,
                            _ => Color::White,
                        };
                        Row::new(vec![
                            Cell::from(connection.clone()).style(Style::default().fg(color)),
                            Cell::from(proto.clone()).style(Style::default().fg(color)),
                            Cell::from(format!("{}", bytes)).style(Style::default().fg(color)),
                        ])
                    })
                    .collect();

                let table = Table::new(rows)
                    .block(Block::default().title("Live Traffic").borders(Borders::ALL))
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Percentage(20),
                        Constraint::Percentage(30),
                    ]);

                frame.render_widget(table, chunks[0]);
            })
            .unwrap();

        // Keyboard
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('f') => {
                        filtering = !filtering;
                        if filtering {
                            println!("Enter filter Regex: ");
                            let mut line = String::new();
                            io::stdin().read_line(&mut line).unwrap();
                            filter = line.trim().to_string();
                        } else {
                            filter.clear();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().unwrap();
}

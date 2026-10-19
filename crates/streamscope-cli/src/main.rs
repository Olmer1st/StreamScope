//! StreamScope CLI boundary: input validation, lifecycle, output, and process codes.
use clap::{Parser, Subcommand, ValueEnum};
use std::{
    io::{self, IsTerminal},
    process::ExitCode,
    sync::{Arc, Mutex},
};
use streamscope_core::{
    capture::{self, Config},
    report, validate_url, State,
};
use streamscope_fixtures::Scenario;
use tokio::sync::watch;

#[derive(Parser)]
#[command(
    name = "streamscope",
    version,
    about = "Inspect SSE framing and client-observed timing. No model API needed."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Start a synthetic SSE server on localhost. No external service is used.
    Demo {
        #[arg(long, value_enum, default_value = "normal")]
        scenario: DemoScenario,
        #[arg(long, default_value_t = 8383)]
        port: u16,
    },
    /// Inspect an endpoint in a live terminal (j/k, space, q).
    Watch(Options),
    /// Check declared assertions. JSON reports never contain event payloads or headers.
    #[command(alias = "probe")]
    Check(Options),
}
#[derive(Clone, Copy, ValueEnum)]
enum DemoScenario {
    Normal,
    Gap,
    Burst,
    Truncated,
    BadUtf8,
}
#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Json,
}
#[derive(clap::Args)]
struct Options {
    #[arg(long)]
    url: String,
    #[arg(long)]
    max_gap_ms: Option<u64>,
    #[arg(long)]
    first_event_ms: Option<u64>,
    #[arg(long)]
    require_last_event: Option<String>,
    #[arg(long, default_value_t = 30000)]
    timeout_ms: u64,
    #[arg(long, default_value_t = 10000)]
    idle_timeout_ms: u64,
    #[arg(long, default_value_t = 0)]
    tolerance_ms: u64,
    #[arg(long, default_value_t = 1048576)]
    max_frame_bytes: usize,
    #[arg(long = "header", short = 'H')]
    headers: Vec<String>,
    #[arg(long, value_enum, default_value = "text")]
    format: Format,
}

#[tokio::main]
async fn main() -> ExitCode {
    // Clap errors may echo argv (including secrets); emit a fixed diagnostic instead.
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error)
            if matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            ) =>
        {
            let _ = error.print();
            return ExitCode::SUCCESS;
        }
        Err(_) => {
            eprintln!("INVALID_INPUT: invalid arguments; run streamscope --help");
            return ExitCode::from(3);
        }
    };
    ExitCode::from(run(cli).await)
}

fn config(options: &Options) -> Result<Config, &'static str> {
    let url = validate_url(&options.url)?;
    if options.timeout_ms == 0
        || options.idle_timeout_ms == 0
        || !(1..=16 * 1024 * 1024).contains(&options.max_frame_bytes)
    {
        return Err("Timeouts must be positive; max-frame-bytes must be 1..16777216");
    }
    if options
        .require_last_event
        .as_ref()
        .is_some_and(|s| s.is_empty() || s.len() > 256 || s.chars().any(char::is_control))
    {
        return Err("Required event name must be 1..256 bytes and contain no controls");
    }
    let mut headers = capture::Headers::default();
    for input in &options.headers {
        let (name, value) = capture::parse_header(input)?;
        // Validation errors never echo values; reports exclude all request headers.
        headers.insert(name, value);
    }
    Ok(Config {
        url,
        headers,
        max_gap_ms: options.max_gap_ms,
        first_event_ms: options.first_event_ms,
        tolerance_ms: options.tolerance_ms,
        require_last_event: options.require_last_event.clone(),
        timeout_ms: options.timeout_ms,
        idle_timeout_ms: options.idle_timeout_ms,
        max_frame_bytes: options.max_frame_bytes,
    })
}

async fn run(cli: Cli) -> u8 {
    let (cancel_tx, cancel_rx) = watch::channel(false);
    let signal_tx = cancel_tx.clone();
    // Register synchronously before printing readiness, so early SIGINT cannot be lost.
    #[cfg(unix)]
    let mut interrupt =
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()) {
            Ok(signal) => signal,
            Err(_) => {
                eprintln!("RUNTIME_ERROR: cannot register interrupt handler");
                return 4;
            }
        };
    let signal = tokio::spawn(async move {
        #[cfg(unix)]
        let received = interrupt.recv().await.is_some();
        #[cfg(not(unix))]
        let received = tokio::signal::ctrl_c().await.is_ok();
        if received {
            let _ = signal_tx.send(true);
        }
    });
    let result = match cli.command {
        Command::Demo { scenario, port } => {
            let scenario = match scenario {
                DemoScenario::Normal => Scenario::Normal,
                DemoScenario::Gap => Scenario::Gap,
                DemoScenario::Burst => Scenario::Burst,
                DemoScenario::Truncated => Scenario::Truncated,
                DemoScenario::BadUtf8 => Scenario::BadUtf8,
            };
            match streamscope_fixtures::bind(port).await {
                Err(_) => {
                    eprintln!("BIND_FAILED: choose a free localhost port");
                    4
                }
                Ok(listener) => {
                    let address = listener.local_addr().expect("bound listener address");
                    println!(
                        "Synthetic SSE: http://{address}/events • scenario {} • Ctrl-C stops",
                        match scenario {
                            Scenario::Normal => "normal",
                            Scenario::Gap => "gap",
                            Scenario::Burst => "burst",
                            Scenario::Truncated => "truncated",
                            Scenario::BadUtf8 => "bad-utf8",
                        }
                    );
                    // Bound graceful shutdown even when a synthetic gap response is still active.
                    let server = streamscope_fixtures::serve(listener, scenario, cancel_rx.clone());
                    let mut stopped = cancel_rx;
                    tokio::select! {
                        result = server => if *stopped.borrow() { 130 } else if result.is_ok() { 0 } else { 4 },
                        _ = stopped.changed() => 130,
                    }
                }
            }
        }
        command => {
            let tui = matches!(command, Command::Watch(_));
            let options = match command {
                Command::Watch(o) | Command::Check(o) => o,
                _ => unreachable!(),
            };
            match config(&options) {
                Err(message) => {
                    eprintln!("INVALID_INPUT: {message}");
                    3
                }
                Ok(config) => {
                    if tui && (!io::stdout().is_terminal() || !io::stdin().is_terminal()) {
                        eprintln!(
                            "INVALID_INPUT: watch needs a terminal; use check --format json for CI"
                        );
                        signal.abort();
                        return 3;
                    }
                    let url = config.url.clone();
                    let state = Arc::new(Mutex::new(State::default()));
                    let task = tokio::spawn(capture::run(config, state.clone(), cancel_rx));
                    let mut ui_code = None;
                    if tui {
                        ui_code = Some(
                            match streamscope_tui::run(state.clone(), cancel_tx.clone()).await {
                                Ok(code) => code,
                                Err(_) => {
                                    eprintln!("TERMINAL_ERROR: retry in a supported terminal");
                                    4
                                }
                            },
                        );
                        let _ = cancel_tx.send(true);
                    }
                    if task.await.is_err() {
                        eprintln!("RUNTIME_ERROR: capture task failed");
                        signal.abort();
                        return 4;
                    }
                    if let Some(code) = ui_code {
                        code
                    } else {
                        let report = report(&state.lock().unwrap(), &url);
                        let result = match options.format {
                            Format::Json => serde_json::to_writer_pretty(io::stdout(), &report)
                                .map_err(io::Error::other),
                            Format::Text => {
                                use std::io::Write;
                                writeln!(io::stdout(), "{}: {} events • {} bytes • first event {:?}ms • max gap {:?}ms\n{}\n{}",
                                    report.verdict, report.metrics.event_count, report.metrics.total_bytes, report.metrics.first_event_ms,
                                    report.metrics.max_observed_gap_ms, report.findings.iter().map(|f| f.code.as_str()).collect::<Vec<_>>().join(", "), report.notes[0])
                            }
                        };
                        if result.is_err() {
                            4
                        } else {
                            report.exit_code
                        }
                    }
                }
            }
        }
    };
    signal.abort();
    result
}

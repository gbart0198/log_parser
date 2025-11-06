use clap::Parser;
use std::borrow::Cow;
use std::fs;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct LogEntry<'a> {
    level: LogLevel,
    message: Cow<'a, str>,
    timestamp: Cow<'a, str>,
}

impl<'a> LogEntry<'a> {
    fn into_owned(self) -> LogEntry<'static> {
        LogEntry {
            timestamp: self.timestamp.into_owned().into(),
            level: self.level,
            message: self.message.into_owned().into(),
        }
    }
}

#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "[DEBUG]" | "[Debug]" => Ok(LogLevel::Debug),
            "[INFO]" | "[Info]" => Ok(LogLevel::Info),
            "[WARNING]" | "[WARN]" | "[Warn]" | "[Warning]" => Ok(LogLevel::Warning),
            "[ERROR]" | "[ERR]" | "[Err]" | "[Error]" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level found {}", s)),
        }
    }
}

struct LogParser;

impl LogParser {
    // assume always happy path first
    fn parse_log_line(line: &str) -> Option<LogEntry<'_>> {
        let parts: Vec<_> = line.splitn(3, " ").collect();
        if parts.len() < 3 {
            return None;
        }
        let timestamp = Cow::Borrowed(parts[0]);
        let level = LogLevel::from_str(parts[1]).unwrap();
        let message = Cow::Borrowed(parts[2]);

        Some(LogEntry {
            level,
            message,
            timestamp,
        })
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to parse logs from
    #[arg(short, long)]
    file_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file_content = std::fs::read_to_string(args.file_name)?;
    let mut logs: Vec<LogEntry> = vec![];

    file_content.split("\n").for_each(|line| {
        if let Some(log) = LogParser::parse_log_line(line) {
            logs.push(log);
        }
    });

    let mut debug_ct = 0;
    let mut info_ct = 0;
    let mut warn_ct = 0;
    let mut error_ct = 0;

    logs.iter().for_each(|log| match log.level {
        LogLevel::Debug => debug_ct += 1,
        LogLevel::Info => info_ct += 1,
        LogLevel::Warning => warn_ct += 1,
        LogLevel::Error => error_ct += 1,
    });

    println!("Number of debug logs: {}", debug_ct);
    println!("Number of info logs: {}", info_ct);
    println!("Number of warn logs: {}", warn_ct);
    println!("Number of error logs: {}", error_ct);

    Ok(())
}

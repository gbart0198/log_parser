use std::borrow::Cow;
use std::fs;
use std::io;

#[derive(Debug)]
struct LogEntry<'a> {
    level: Cow<'a, str>,
    message: Cow<'a, str>,
    timestamp: Cow<'a, str>,
}

struct LogParser;

impl LogParser {
    fn parse_log_line(line: &str) -> Option<LogEntry<'_>> {
        todo!()
    }
}
fn main() {}

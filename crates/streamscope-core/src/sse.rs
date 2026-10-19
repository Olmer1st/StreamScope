//! Incremental SSE framing, independent of HTTP reads and async runtimes.
use thiserror::Error;

/// A completed SSE dispatch; IDs persist across dispatches per SSE rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub event_type: String,
    pub id: String,
    pub data: String,
    pub frame_bytes: usize,
}

/// Strict diagnostics contain no payload bytes.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("INVALID_UTF8")]
    InvalidUtf8,
    #[error("FRAME_TOO_LARGE")]
    FrameTooLarge,
    #[error("INCOMPLETE_FRAME")]
    IncompleteFrame,
}

/// A bounded line-oriented decoder. CR dispatches immediately; following LF is ignored.
pub struct Decoder {
    line: Vec<u8>,
    data: String,
    event_type: String,
    id: String,
    has_data: bool,
    skip_lf: bool,
    first_line: bool,
    frame_bytes: usize,
    limit: usize,
    /// Completed comment lines; comments never dispatch an event.
    pub comments: u64,
}

impl Decoder {
    /// Construct with a hard per-frame byte cap (including ignored fields/comments).
    pub fn new(limit: usize) -> Self {
        Self {
            line: Vec::new(),
            data: String::new(),
            event_type: String::new(),
            id: String::new(),
            has_data: false,
            skip_lf: false,
            first_line: true,
            frame_bytes: 0,
            limit,
            comments: 0,
        }
    }

    /// Consume one byte. Returning at most one dispatch avoids chunk-sized allocations.
    pub fn push(&mut self, byte: u8) -> Result<Option<Event>, ParseError> {
        if self.skip_lf {
            self.skip_lf = false;
            if byte == b'\n' {
                return Ok(None);
            }
        }
        if self.frame_bytes >= self.limit {
            return Err(ParseError::FrameTooLarge);
        }
        self.frame_bytes += 1;
        if byte != b'\r' && byte != b'\n' {
            self.line.push(byte);
            return Ok(None);
        }
        self.skip_lf = byte == b'\r';
        let bytes = std::mem::take(&mut self.line);
        let mut line = std::str::from_utf8(&bytes).map_err(|_| ParseError::InvalidUtf8)?;
        if self.first_line {
            line = line.strip_prefix('\u{feff}').unwrap_or(line);
            self.first_line = false;
        }
        if line.is_empty() {
            let event = if self.has_data {
                self.data.pop(); // Remove the final data-field newline, including empty data.
                Some(Event {
                    event_type: if self.event_type.is_empty() {
                        "message".into()
                    } else {
                        std::mem::take(&mut self.event_type)
                    },
                    id: self.id.clone(),
                    data: std::mem::take(&mut self.data),
                    frame_bytes: self.frame_bytes,
                })
            } else {
                None
            };
            self.data.clear();
            self.event_type.clear();
            self.has_data = false;
            self.frame_bytes = 0;
            return Ok(event);
        }
        if line.starts_with(':') {
            self.comments += 1;
            return Ok(None);
        }
        let (name, value) = line.split_once(':').unwrap_or((line, ""));
        let value = value.strip_prefix(' ').unwrap_or(value);
        match name {
            "data" => {
                self.data.push_str(value);
                self.data.push('\n');
                self.has_data = true;
            }
            "event" => self.event_type = value.into(),
            "id" if !value.contains('\0') => self.id = value.into(),
            _ => {} // retry and unknown fields are ignored; reconnect is outside MVP.
        }
        Ok(None)
    }

    /// EOF never dispatches pending data. Invalid UTF-8 has precedence over incomplete framing.
    pub fn finish(&self) -> Result<(), ParseError> {
        std::str::from_utf8(&self.line).map_err(|_| ParseError::InvalidUtf8)?;
        if self.frame_bytes > 0 {
            Err(ParseError::IncompleteFrame)
        } else {
            Ok(())
        }
    }
}

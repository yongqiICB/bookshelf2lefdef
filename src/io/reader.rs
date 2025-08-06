use std::io::{self, BufRead, BufReader, Read};

pub struct TokenReader<R: Read> {
    source: BufReader<R>,
    buffer: Vec<u8>,
    current_pos: usize,
    total_bytes_read: usize,
}

impl<R: Read> TokenReader<R> {
    pub fn new(source: R) -> Self {
        TokenReader {
            source: BufReader::new(source),
            buffer: Vec::new(),
            current_pos: 0,
            total_bytes_read: 0,
        }
    }

    pub fn swallow_line(&mut self) -> io::Result<usize> {
        self.buffer.clear();
        self.current_pos = 0;
        let bytes_read = self.source.read_until(b'\n', &mut self.buffer)?;
        self.total_bytes_read += bytes_read;
        Ok(bytes_read)
    }

    pub fn peek_token(&mut self) -> io::Result<Option<&str>> {
        loop {
            while self.current_pos < self.buffer.len() && self.buffer[self.current_pos].is_ascii_whitespace() {
                self.current_pos += 1;
            }
            if self.current_pos < self.buffer.len() {
                let start = self.current_pos;
                let mut end = start;
                while end < self.buffer.len() && !self.buffer[end].is_ascii_whitespace() {
                    end += 1;
                }
                let token_slice = &self.buffer[start..end];
                let token_str = std::str::from_utf8(token_slice)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                return Ok(Some(token_str))
            }
            let bytes_read = self.swallow_line()?;
            if bytes_read == 0 {
                return Ok(None)
            }
        }
    }

    pub fn next_token(&mut self) -> io::Result<Option<&str>> {
        loop {
            while self.current_pos < self.buffer.len() && self.buffer[self.current_pos].is_ascii_whitespace() {
                self.current_pos += 1;
            }

            if self.current_pos < self.buffer.len() {
                let start = self.current_pos;
                let mut end = start;
                while end < self.buffer.len() && !self.buffer[end].is_ascii_whitespace() {
                    end += 1;
                }
                let token_slice = &self.buffer[start..end];
                let token_str = std::str::from_utf8(token_slice)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                self.current_pos = end;
                return Ok(Some(token_str));
            }
            let bytes_read = self.swallow_line()?;            
            if bytes_read == 0 {
                return Ok(None);
            }
        }
    }
}
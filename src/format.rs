pub fn format_as_ascii_and_hex(buffer: &[u8]) -> String {
    buffer
        .chunks(16)
        .map(|bytes_chunk| {
            let ascii = bytes_chunk
                .iter()
                .map(|c| Byte(*c).as_char())
                .collect::<String>();

            let hex = bytes_chunk
                .iter()
                .map(|c| Byte(*c).as_hex())
                .fold(String::new(), |s, hex| s + &hex);

            return format!("{:<16}   {}", ascii, hex);
        })
        .fold(String::new(), |source, chars| source + &chars + "\n")
}

#[derive(Copy, Clone)]
struct Byte(u8);

impl Byte {
    fn category(self) -> ByteCategory {
        if self.0 == 0x00 {
            ByteCategory::Null
        } else if self.0.is_ascii_alphanumeric()
            || self.0.is_ascii_punctuation()
            || self.0.is_ascii_graphic()
        {
            ByteCategory::AsciiPrintable
        } else if self.0.is_ascii_whitespace() {
            ByteCategory::AsciiWhitespace
        } else if self.0.is_ascii() {
            ByteCategory::AsciiOther
        } else {
            ByteCategory::NonAscii
        }
    }

    fn as_char(self) -> char {
        match self.category() {
            ByteCategory::Null => '0',
            ByteCategory::AsciiPrintable => self.0 as char,
            ByteCategory::AsciiWhitespace if self.0 == 0x20 => ' ',
            ByteCategory::AsciiWhitespace => '_',
            ByteCategory::AsciiOther => '•',
            ByteCategory::NonAscii => '×',
        }
    }
    fn as_hex(self) -> String {
        format!("{:02x} ", self.0)
    }
}

enum ByteCategory {
    Null,
    AsciiPrintable,
    AsciiWhitespace,
    AsciiOther,
    NonAscii,
}

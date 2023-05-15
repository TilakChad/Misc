// Well this might end up being actually an assembler too
// Labels and calls can't be identified in a single pass, so gotta do a double-pass just for syntax highlighting ???

#[derive(Debug)]
struct Color(&'static str);

// Size of each escaped color sequence is 5
const Red: Color = Color("\x1b[31m");
const Blue: Color = Color("\x1b[34m");
const Green: Color = Color("\x1b[32m");
const Yellow: Color = Color("\x1b[33m");
const Magenta: Color = Color("\x1b[90m"); // its actually gray color being used now instead of Magenta color
const Cyan: Color = Color("\x1b[96m");
const Default: Color = Color("\x1b[0m");

#[derive(Debug)]
struct ColorInfo {
    registers: Color,
    keywords: Color,
    immediate: Color,
    labels: Color,
    comments: Color,
}

#[derive(Debug)]
struct SourceMetadata<'a> {
    colors_for: ColorInfo,
    keywords: Vec<&'a str>,
    registers: Vec<&'a str>,
    indexed_labels: Vec<(&'a str, u32)>,

    output_src: Vec<String>, // each line represented in each vector
}

#[derive(Debug)]
struct SourceInfo {
    // Scan the source to include information about the occurence of the tokens along with their types
}

// Form the token stream and parse the whole file as required
// Well, well, might need to write whole tokenizer and parser for this.

// TODO:: Seperate str on a seperate field
#[derive(PartialEq)]
enum Token<'a> {
    Keyword(&'a str), // or assembly instruction
    Register(&'a str),
    Label(&'a str),
    Comments(&'a str),
    AlphaNumeric(&'a str),
    WhiteSpaceChars(&'a str),
    Colon,
    Comma,
    Ampersand,
    Hash,
    Asterisk,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    None,
}

struct Tokenizer<'a> {
    pos: usize,
    length: usize,
    line: u32,
    current_token: Token<'a>,
    next_token: Token<'a>,
    raw_bytes: &'a [u8],
}

fn is_ascii_alpha(v: u8) -> bool {
    (v >= b'a' && v <= b'z') || (v >= b'A' && v <= b'Z')
}

fn is_ascii_num(v: u8) -> bool {
    v >= b'0' && v <= b'9'
}

impl<'a> Tokenizer<'a> {
    fn new(src: &'a str) -> Self {
        Tokenizer {
            pos: 0,
            line: 0,
            length: src.len(),
            raw_bytes: src.as_bytes(),
            current_token: Token::None,
            next_token: Token::None,
        }
    }

    fn init(&mut self) {
        self.next_token = self.next_token_internal();
    }

    fn next_token(&mut self) -> Token<'a> {
        let upcoming_token = self.next_token_internal();
        return std::mem::replace(&mut self.next_token, upcoming_token);
    }

    fn lookahead_token(&mut self) -> &Token<'a> {
        return &self.next_token;
    }

    fn next_token_internal(&mut self) -> Token<'a> {
        // continue until next character is found
        let start = self.pos;

        // skip any whitespace characters
        let mut whitespace_chars = false;
        while self.pos < self.length
            && (self.raw_bytes[self.pos] == b' '
                || self.raw_bytes[self.pos] == b'\t'
                || self.raw_bytes[self.pos] == b'\n')
        {
            whitespace_chars = true;
            self.pos = self.pos + 1;
        }

        if whitespace_chars {
            return Token::WhiteSpaceChars(
                std::str::from_utf8(&self.raw_bytes[start..self.pos]).unwrap(),
            );
        }

        if self.pos >= self.length {
            return Token::None;
        }
        // TODO :: Refactor all the codes with pattern matching
        // TODO :: Append with an very unlikely character to reduce checks and speed up the parsing process
        if is_ascii_alpha(self.raw_bytes[self.pos]) {
            self.pos = self.pos + 1;
            // continue until weird characters starts to be obtained
            // the token is probably keywords,
            while self.pos < self.length
                && (is_ascii_alpha(self.raw_bytes[self.pos])
                    || is_ascii_num(self.raw_bytes[self.pos]))
            {
                self.pos = self.pos + 1;
            }
            let ref_val = std::str::from_utf8(&self.raw_bytes[start..self.pos]).unwrap();
            return Token::AlphaNumeric(ref_val);
        }

        let symbols = match self.raw_bytes[self.pos] {
            b':' => Some(Token::Colon),
            b',' => Some(Token::Comma),
            b'*' => Some(Token::Asterisk),
            b'&' => Some(Token::Ampersand),
            b'[' => Some(Token::LeftBracket),
            b']' => Some(Token::RightBracket),
            b'(' => Some(Token::LeftParen),
            b')' => Some(Token::RightParen),
            _ => None,
        };
        if symbols.is_some() {
            self.pos = self.pos + 1;
            return symbols.unwrap();
        }

        return Token::None;
    }
}

impl<'a> SourceMetadata<'a> {
    fn check_if_keyword(&self, tok: &str) -> bool {
        self.keywords.contains(&tok)
    }

    fn check_if_register(&self, tok: &str) -> bool {
        self.registers.contains(&tok)
    }

    fn print_final(&self) {
        for x in &self.output_src {
            println!("{}", x);
        }
    }
}

fn print_source<'a>(token_stream: &mut Tokenizer<'a>, src_metadata: &mut SourceMetadata<'a>) {
    let mut token = token_stream.next_token();
    println!("\n\nPrinitng formated source : ");

    while token != Token::None {
        match token {
            Token::AlphaNumeric(x) => {
                if src_metadata.check_if_register(x) {
                    std::fmt::write(
                        src_metadata.output_src.last_mut().unwrap(),
                        format_args!("{}{x}{}", src_metadata.colors_for.registers.0, Default.0),
                    )
                    .unwrap();
                } else if src_metadata.check_if_keyword(x) {
                    std::fmt::write(
                        src_metadata.output_src.last_mut().unwrap(),
                        format_args!("{}{x}{}", src_metadata.colors_for.keywords.0, Default.0),
                    )
                    .unwrap();
                } else {
                    if token_stream.lookahead_token() == &Token::Colon {
                        // its an offset, write it and index into the source too
                        let row = src_metadata.indexed_labels.len();
                        src_metadata.indexed_labels.push((x, row as u32));
                        std::fmt::write(
                            src_metadata.output_src.last_mut().unwrap(),
                            format_args!("{x}:"),
                        )
                        .unwrap();
                        token_stream.next_token();
                    } else {
                        std::fmt::write(
                            src_metadata.output_src.last_mut().unwrap(),
                            format_args!("{x}"),
                        )
                        .unwrap();
                    }
                }
            }

            // TODO :: Replace with bindings
            Token::Colon => {
                src_metadata.output_src.last_mut().unwrap().push(':');
            }
            Token::Comma => {
                src_metadata.output_src.last_mut().unwrap().push(',');
            }
            Token::LeftBracket => {
                src_metadata.output_src.last_mut().unwrap().push('[');
            }
            Token::RightBracket => {
                src_metadata.output_src.last_mut().unwrap().push(']');
            }
            Token::LeftParen => {
                src_metadata.output_src.last_mut().unwrap().push('(');
            }
            Token::RightParen => {
                src_metadata.output_src.last_mut().unwrap().push(')');
            }

            Token::WhiteSpaceChars(x) => {
                // iterate over new lines
                for ch in x.bytes() {
                    if ch == b'\n' {
                        src_metadata.output_src.push(String::new());
                    } else {
                        src_metadata.output_src.last_mut().unwrap().push(ch as char);
                    }
                }
            }
            _ => {
                println!("Invalid symbol/literals found");
            }
        }
        token = token_stream.next_token();
    }
}

fn main() {
    println!("Formatting asm source code : ");

    let color_info = ColorInfo {
        keywords: Blue,
        registers: Red,
        immediate: Yellow,
        labels: Cyan,
        comments: Magenta,
    };

    let mut src_metadata = SourceMetadata {
        keywords: vec!["mov", "lea", "sti", "cli", "xor", "and", "or", "not"],
        registers: vec![
            "eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp", "ss", "es", "ds", "cs",
        ],
        colors_for: color_info,
        indexed_labels: Vec::new(),
        output_src: Vec::new(),
    };

    src_metadata.output_src.push(String::new());

    let source_code = String::from("mov eax, ebx\nlea ebx, [offset]\noffset:");

    let mut tokenizer = Tokenizer::new(&source_code);
    tokenizer.init();

    print_source(&mut tokenizer, &mut src_metadata);

    println!("Printing the final version : \n");
    src_metadata.print_final();
}

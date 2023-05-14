// Well this might end up being actually an assembler too
// Labels and calls can't be identified in a single pass, so gotta do a double-pass just for syntax highlighting ???  

#[derive(Debug)]
struct Color(&'static str);

// Size of each escaped color sequence is 5 
const Red: Color       = Color("\x1b[31m");
const Blue: Color      = Color("\x1b[34m");
const Green: Color     = Color("\x1b[32m");
const Yellow: Color    = Color("\x1b[33m");
const Magenta: Color   = Color("\x1b[90m"); // its actually gray color being used now instead of Magenta color
const Cyan: Color      = Color("\x1b[96m");
const Default: Color   = Color("\x1b[0m");

#[derive(Debug)]
struct ColorInfo {
    registers : Color,
    keywords  : Color,
    immediate : Color,
    labels    : Color,
    comments  : Color,
}

#[derive(Debug)]
struct SourceMetadata<'a> {
    colors_for	      : ColorInfo,
    keywords	      : Vec<&'a str>,
    registers	      : Vec<&'a str>,
    labels	      : Vec<(&'a str, u32)>,

    output_src 	      : Vec<String> // each line represented in each vector 
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
    fn init(src: &'a str) -> Self {
        Tokenizer {
            pos: 0,
            line: 0,
            length: src.len(),
            raw_bytes: src.as_bytes(),
            current_token: Token::None,
            next_token: Token::None,
        }
    }

    fn next_token(&mut self) -> Token<'a> {
        // continue until next character is found
        let start = self.pos;

        // skip any whitespace characters
        let mut whitespace_chars = false;
        while self.pos < self.length && (self.raw_bytes[self.pos] == b' '
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
            // decide if the obtained string is any keywords
            // check if the source code actually compiles
            // This function can't fail
            println!(
                "String parsed so far : {} at {}.",
                std::str::from_utf8(&self.raw_bytes[start..self.pos]).unwrap(),
                self.pos
            );
            let ref_val = std::str::from_utf8(&self.raw_bytes[start..self.pos]).unwrap();
            if check_if_keyword(ref_val) {
                return Token::Keyword(ref_val);
            }
            if check_if_register(ref_val) {
                return Token::Register(ref_val);
            }
            // can't check for label without next string
            return Token::AlphaNumeric(ref_val);
        }

	let symbols =  match self.raw_bytes[self.pos] {
            b':' =>  Some(Token::Colon),
            b',' =>  Some(Token::Comma),
            b'*' =>  Some(Token::Asterisk),
            b'&' =>  Some(Token::Ampersand),
           _     => None
        };
	if symbols.is_some() {
           self.pos = self.pos + 1;
	   return symbols.unwrap();
	}

        return Token::None;
    }

    fn lookahead_token(&mut self) -> Token<'a> {
        return Token::None;
    }
}

fn check_if_keyword(tok: &str) -> bool {
    false
}

fn check_if_register(tok: &str) -> bool {
    false
}

fn check_if_label(tok: &str) -> bool {
    false
}



fn print_source(token_stream: &mut Tokenizer, src_metadata : &mut SourceMetadata) {
    let mut token = token_stream.next_token();
    println!("\n\nPrinitng formated source : ");
    
    while token != Token::None {
        match token {
            Token::Register(x) => {
       	        println!(" Reg : {}\n", x);
		println!("{}{x}{}", src_metadata.colors_for.registers.0,Default.0); 
            }
            Token::AlphaNumeric(x) => {
                if token_stream.lookahead_token() == Token::Colon {
                    println!("Found label : {}\n", x);
                } else {
                    println!("Plain alpha numeric token");
		    println!("{}{x}{}", Red.0, Default.0); 
		    
                }
            }

            Token::Colon => {
                println!("colon");
            }
            Token::Comma => {
                println!("comma");
            }
	    Token::WhiteSpaceChar(x) => {
	        // iterate over new lines
		for ch in x {
		    if ch == b'\n' {
		       src_metadata.output_src.push(String::new()); 
		    }
		}
	    }
            _ => {}
        }
        token = token_stream.next_token();
    }
}

fn main() {
    println!("Formatting asm source code : ");

    let color_info = ColorInfo {
    	keywords   : Blue,
	registers  : Red,
	immediate  : Yellow,
	labels     : Cyan,
	comments   : Magenta
        };

    
    let src_metadata = SourceMetadata {
    	keywords   : vec!("mov", "lea", "sti", "cli", "xor","and", "or", "not"), 
	registers  : vec!("eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp", "ss", "es", "ds", "cs"), 
	colors_for : color_info, 
	labels     : Vec::new()
        }; 	 

    let source_code = String::from("mov eax, ebx\n");
    
    let mut tokenizer = Tokenizer::init(&source_code);
    print_source(&mut tokenizer,&mut src_metadata);
}

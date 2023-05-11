#[derive(Debug)]
struct Color(&'static str);

const Red     : Color = Color("\033[03m");
const Blue    : Color = Color("");
const Green   : Color = Color("");
const Yellow  : Color = Color("");
const Magenta : Color = Color("");
const Cyan    : Color = Color("");

#[derive(Debug)]
struct ColorInfo {
       registers : Color,
       keywords  : Color,
       immediate : Color,
       lablesl   : Color, 
       comments  : Color 
}

#[derive(Debug)]
struct Metadata {
       colors_for : ColorInfo,
       keywords   : Vec<String>,
       registers  : Vec<String>,  
}

#[derive(Debug)]
struct SourceInfo {
       // Scan the source to include information about the occurence of the tokens along with their types 
}

// Form the token stream and parse the whole file as required
// Well, well, might need to write whole tokenizer and parser for this. 

enum Token<'a> {
     Keyword(&'a str), // or assembly instruction 
     Register(&'a str),
     Label(&'a str),
     Comments(&'a str),
     None
}

enum ParsedToken<'a> {
     AlphaNumeric(&'a str),
     Colon,
     Comma,
     LeftBracket,
     RightBrackt,
     None 
}

struct Tokenizer<'a>  {
       pos	   : usize,
       length 	   : usize, 
       raw_bytes   : &'a [u8],
       current_token : Token<'a>,
       next_token    : Token<'a>,
}

fn is_ascii_alpha(v : u8) -> bool {
   (v >= b'a' && v <= b'z') || (v >= b'A' && v <= b'Z') 
}

fn is_ascii_num(v : u8) -> bool {
   v >= b'0' && v <= b'9' 
}

impl<'a> Tokenizer<'a> {
     fn init(src : &'a str) -> Self {
     	Tokenizer {
		  pos		: 0,
		  length 	: src.len(),
		  raw_bytes 	: src.as_bytes(),
		  current_token : Token::None,
		  next_token    : Token::None
	}
     }

     fn next_token(&mut self) -> Token<'a> {
     	    // continue until next character is found
	    // should start with alphabet
	    let parsed_token = ParsedToken::None;
	    let start 	     = self.pos;
	    
	    // TODO :: Append with an very unlikely character to reduce checks and speed up the parsing process
	    if is_ascii_alpha(self.raw_bytes[self.pos]) {
		self.pos = self.pos + 1;
		// continue until weird characters starts to be obtained
		// the token is probably keywords,
		while self.pos < self.length && (is_ascii_alpha(self.raw_bytes[self.pos]) || is_ascii_num(self.raw_bytes[self.pos])) {
		      self.pos = self.pos + 1; 
		}
		// decide if the obtained string is any keywords
		// check if the source code actually compiles
		println!("String parsed so far : {}.", std::str::from_utf8(&self.raw_bytes[start..self.pos]).unwrap());
	    }

	    return Token::None; 
     }

     fn lookahead_token(&mut self) -> Token<'a>{
     	return Token::None; 
     }
}

fn check_if_keyword() {
}

fn check_if_register() {
}

fn check_if_label() {
}


fn print_source(src : String) {
   println!("{}",src); 
}

fn main() {
       println!("Formatting asm source code : ");
       let source_code = String::from("mov eax, ebx\n");
       // print_source(source_code);
       let mut tokenizer = Tokenizer::init(&source_code);
       tokenizer.next_token(); 
       println!("Hello, world!");
}


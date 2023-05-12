#[derive(Debug)]
struct Color(&'static str);

const Red     : Color = Color("\033[31m");
const Blue    : Color = Color("\033[34m");
const Green   : Color = Color("\033[32m");
const Yellow  : Color = Color("\033[33m");
const Magenta : Color = Color("\033[90m");  // its actually green color used now 
const Cyan    : Color = Color("\033[96m");
const Default : Color = Color("\033[0m");

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
       labels     : Vec<(String, u32)>
}

#[derive(Debug)]
struct SourceInfo {
       // Scan the source to include information about the occurence of the tokens along with their types 
}

// Form the token stream and parse the whole file as required
// Well, well, might need to write whole tokenizer and parser for this. 

// TODO:: Seperate str on a seperate field 
enum Token<'a> {
     Keyword(&'a str), // or assembly instruction 
     Register(&'a str),
     Label(&'a str),
     Comments(&'a str),
     AlphaNumeric(&'a str),
     Colon,
     Comma,
     Ampersand,
     Hash,
     Asterisk, 
     None
}

struct Tokenizer<'a>  {
       pos	   : usize,
       length 	   : usize,
       line        : u32, 
       current_token : Token<'a>,
       next_token    : Token<'a>,
       raw_bytes   : &'a [u8],
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
		  line 		: 0, 
		  length 	: src.len(),
		  raw_bytes 	: src.as_bytes(),
		  current_token : Token::None,
		  next_token    : Token::None
	}
     }

     fn next_token(&mut self) -> Token<'a> {
     	    // continue until next character is found
	    // should start with alphabet
	    let start 	     = self.pos;


	    // skip any whitespace characters
	    while self.raw_bytes[self.pos] == b' ' || self.raw_bytes[self.pos] == b'\t' || self.raw_bytes[self.pos] == b'\n' {
	    	  self.pos = self.pos + 1; 
	    }

	    // TODO :: Refactor all the codes with pattern matching 
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
		// This function can't fail 
		println!("String parsed so far : {}.", std::str::from_utf8(&self.raw_bytes[start..self.pos]).unwrap());
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

	    match self.raw_bytes[self.pos] {
	    	  b':' => {
		       return Token::Colon;
		       }
		  b',' => {
		       return Token::Comma;
		       }
		  b'*' => {
		       return Token::Asterisk;
		       }
		  b'&' => {
		       return Token::Ampersand;
		       }
		  default => {
		  }
	    }
	    	    
	    return Token::None; 
     }

     fn lookahead_token(&mut self) -> Token<'a>{
     	return Token::None; 
     }
}

fn check_if_keyword(tok : &str) -> bool {
   false
}

fn check_if_register(tok : &str) -> bool {
   false
}

fn check_if_label(tok : &str) -> bool {
   false
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


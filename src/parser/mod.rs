use crate::tokenizer::offset_to_location::OffsetToLocation;
use std::collections::HashMap;

pub struct Parser {
    source: String,
    filename: String,
    need_positions: bool,
    on_parse_error: Option<Box<dyn Fn(String, Option<Node>)>>,
    on_parse_error_throw: bool,
    context: HashMap<String, Box<dyn Fn(&mut Parser, &ParseOptions) -> Node>>,
    config: ParseConfig,
    location_map: OffsetToLocation,
    token_stream: TokenStream,
}

pub struct ParseOptions {
    pub context: String,
    pub on_comment: Option<Box<dyn Fn(String, Location)>>,
    pub on_token: Option<Box<dyn Fn(Token)>>,
    pub positions: bool,
    pub filename: String,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
    pub parse_atrule_prelude: bool,
    pub parse_rule_prelude: bool,
    pub parse_value: bool,
    pub parse_custom_property: bool,
}

pub struct ParseConfig {
    pub features: HashMap<String, bool>,
    pub scope: HashMap<String, bool>,
    pub atrule: HashMap<String, Box<dyn Fn(&mut Parser) -> Node>>,
    pub pseudo: HashMap<String, Box<dyn Fn(&mut Parser) -> Node>>,
    pub node: HashMap<String, Box<dyn Fn(&mut Parser) -> Node>>,
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

pub struct Location {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

pub struct Node;

impl Parser {
    pub fn new(config: ParseConfig) -> Self {
        Parser {
            source: String::new(),
            filename: "<unknown>".to_string(),
            need_positions: false,
            on_parse_error: None,
            on_parse_error_throw: false,
            context: HashMap::new(),
            config,
            location_map: OffsetToLocation::new(),
            token_stream: TokenStream::new(),
        }
    }

    pub fn parse(&mut self, source: String, options: ParseOptions) -> Node {
        self.source = source;
        self.filename = options.filename.clone();
        self.need_positions = options.positions;
        self.on_parse_error = options
            .on_comment
            .map(|f| Box::new(f) as Box<dyn Fn(String, Option<Node>)>);
        self.on_parse_error_throw = false;

        self.token_stream.set_source(&self.source);
        self.location_map
            .set_source(&self.source, options.offset, options.line, options.column);

        if !self.context.contains_key(&options.context) {
            panic!("Unknown context `{}`", options.context);
        }

        if let Some(on_token) = options.on_token {
            self.token_stream.for_each_token(on_token);
        }

        if let Some(on_comment) = options.on_comment {
            self.token_stream.for_each_token(|token| {
                if token.token_type == TokenType::Comment {
                    let loc = self.location_map.get_location(token.start, token.end);
                    let value = if self.source[token.end - 2..token.end] == "*/" {
                        self.source[token.start + 2..token.end - 2].to_string()
                    } else {
                        self.source[token.start + 2..token.end].to_string()
                    };
                    on_comment(value, loc);
                }
            });
        }

        let ast = self.context[&options.context](self, &options);

        if !self.token_stream.eof() {
            self.error("Unexpected end of input");
        }

        ast
    }

    fn error(&self, message: &str) {
        panic!("{}", message);
    }
}

pub struct TokenStream;

impl TokenStream {
    pub fn new() -> Self {
        TokenStream
    }

    pub fn set_source(&mut self, source: &str) {
        // Set source logic
    }

    pub fn for_each_token<F>(&self, mut callback: F)
    where
        F: FnMut(Token),
    {
        // Iterate over tokens and call the callback
    }

    pub fn eof(&self) -> bool {
        // Check if end of file
        true
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Comment,
    AtKeyword,
    Semicolon,
    LeftCurlyBracket,
    RightCurlyBracket,
    EOF,
}

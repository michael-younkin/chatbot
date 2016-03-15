extern crate rand;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::io::BufRead;
use std::io::Write;
use std::io;
use std::collections::HashSet;
use rand::Rng;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    InputStart,
    Punct,
    Word,
    Space,
    InputEnd,
}

impl TokenKind {
    fn as_str(self) -> &'static str {
        match self {
            TokenKind::InputStart => "InputStart",
            TokenKind::InputEnd => "InputEnd",
            TokenKind::Punct => "Punct",
            TokenKind::Word => "Word",
            TokenKind::Space => "Space",
        }
    }

    fn with_name(s: &str) -> TokenKind {
        match s {
            "InputStart" => TokenKind::InputStart,
            "InputEnd" => TokenKind::InputEnd,
            "Punct" => TokenKind::Punct,
            "Word" => TokenKind::Word,
            "Space" => TokenKind::Space,
            _ => panic!("Invalid name"),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Token {
    value: Option<String>,
    kind: TokenKind
}

impl Token {
    fn new(kind: TokenKind) -> Token {
        Token {
            value: None,
            kind: kind
        }
    }

    fn with_value(kind: TokenKind, value: &str) -> Token {
        // TODO consider panicing if kind shouldn't have a value
        Token {
            value: Some(value.to_owned()),
            kind: kind
        }
    }

    fn is_not_end(&self) -> bool {
        match self.kind {
            TokenKind::InputEnd => false,
            _ => true
        }
    }

    fn value(&self) -> Option<&str> {
        self.value.as_ref().map(String::as_ref)
    }
}

struct TokenContext {
    token: Token,
    next_tokens: HashMap<Token, u32>,
    total_weight: u32,
}

impl TokenContext {
    fn new(token: &Token) -> TokenContext {
        TokenContext {
            token: token.to_owned(),
            next_tokens: HashMap::new(),
            total_weight: 0
        }
    }

    fn total_weight(&self) -> u32 {
        self.total_weight
    }

    fn add_next(&mut self, token: &Token) {
        let count = self.next_tokens.entry(token.to_owned()).or_insert(0);
        *count += 1;
        self.total_weight += 1;
    }

    fn get_weighted(&self, mut index: u32) -> &Token {
        if index >= self.total_weight {
            panic!("Index out of bounds.");
        }
        for (k,v) in self.next_tokens.iter() {
            if index < *v {
                return k
            } else {
                index -= *v;
            }
        }
        // This should never happen if we count total_weight properly.
        panic!("Index out of bounds.");
    }
}

struct KnowledgeBase {
    tokens: HashMap<Token, TokenContext>
}

impl KnowledgeBase {
    fn new() -> KnowledgeBase {
        KnowledgeBase {
            tokens: HashMap::new()
        }
    }

    fn add_pair(&mut self, left: &Token, right: &Token) {
        let left_context = self.tokens.entry(left.to_owned())
            .or_insert_with(|| TokenContext::new(left));
        left_context.add_next(right);
    }

    fn num_tokens(&self) -> usize {
        self.tokens.len()
    }

    fn gen_output<Rand>(&self, rng: &mut Rand) -> String where Rand: Rng{
        let mut out = String::new();
        let mut last_token = &Token::new(TokenKind::InputStart);
        while last_token.is_not_end() {
            // Unwrap because the only time this will happen is if the database got corrupted or if
            // we don't have anything inserted yet. TODO add better behavior here later (maybe
            // default contents of the knowledgebase?).
            let context = self.tokens.get(last_token).unwrap();
            last_token = context.get_weighted(rng.gen_range(0, context.total_weight()));
            if let Some(s) = last_token.value() {
                out.push_str(s);
                out.push_str(" ");
            }
        }
        out.pop();
        out
    }
}

const TOKEN_PATTERNS: [(TokenKind, &'static str); 3] = [
    (TokenKind::Space, "\\s"),
    (TokenKind::Punct, ".:;?!,"),
    (TokenKind::Word, "\\w'")
];

lazy_static! {
    static ref TOKEN_REGEX: Regex = {
        let mut s = String::new();
        s.push_str("^(");
        for &(kind, chars) in TOKEN_PATTERNS.iter() {
            s.push_str(&format!("(?P<{}>[{}]+)|", kind.as_str(), chars));
        }
        s.pop();
        s.push_str(")(?P<rest>.*)$");
        Regex::new(&s).unwrap()
    };
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut curr = s.clone();
    let mut tokens: Vec<Token> = vec![Token::new(TokenKind::InputStart)];
    'capture_loop: while curr.len() > 0 {
        let caps = TOKEN_REGEX.captures(curr).expect("Unexpected character encountered.");
        for &(kind, _) in TOKEN_PATTERNS.iter() {
            if let Some(s) = caps.name(kind.as_str()) {
                if kind != TokenKind::Space {
                    tokens.push(Token::with_value(kind, s));
                }
                curr = caps.name("rest").expect("Regex error.");
                continue 'capture_loop;
            }
        }
        panic!("Unexpected character encountered.");
    }
    tokens.push(Token::new(TokenKind::InputEnd));
    tokens
}

fn main() {
    let mut rng = rand::thread_rng();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut kb = KnowledgeBase::new();
    print!("User: ");
    stdout.flush().unwrap();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut tokens_iter = tokenize(&line).into_iter();
        let mut left = tokens_iter.next().unwrap();
        while let Some(right) = tokens_iter.next() {
            kb.add_pair(&left, &right);
            left = right;
        }

        println!(">> {}", kb.gen_output(&mut rng));

        print!("User: ");
        stdout.flush().unwrap();
    }
}

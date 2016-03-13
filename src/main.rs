extern crate rand;

use std::io::BufRead;
use std::io::Write;
use std::io;
use std::collections::HashSet;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Token {
    InputStart,
    Word(String),
    InputEnd
}

impl Token {
    fn is_not_end(&self) -> bool {
        match self {
            &Token::InputEnd => false,
            _ => true
        }
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
        let mut last_token = &Token::InputStart;
        while last_token.is_not_end() {
            // Unwrap because the only time this will happen is if the database got corrupted or if
            // we don't have anything inserted yet. TODO add better behavior here later (maybe
            // default contents of the knowledgebase?).
            let context = self.tokens.get(last_token).unwrap();
            last_token = context.get_weighted(rng.gen_range(0, context.total_weight()));
            if let &Token::Word(ref s) = last_token {
                out.push_str(s);
                out.push_str(" ");
            }
            println!("{}", &out);
        }
        out.pop();
        out
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut kb = KnowledgeBase::new();
    print!("User: ");
    stdout.flush().unwrap();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let token_pairs = line.split_whitespace().map(|word| Token::Word(word.to_owned()));
        let mut left = Token::InputStart;
        for right in token_pairs {
            kb.add_pair(&left, &right);
            left = right;
        }
        kb.add_pair(&left, &Token::InputEnd);

        println!(">> {}", kb.gen_output(&mut rng));

        print!("User: ");
        stdout.flush().unwrap();
    }
}

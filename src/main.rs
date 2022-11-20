use std::{io::{self, Write}, collections::HashMap};

#[derive(Debug)]
enum Token {
    Add,
    Subtract,
    Assign,
    Print,
    Name(String),
    Number(i64)
}

struct VirtualMachine {
    stack: Vec<Token>,
    names: HashMap<String,i64>
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine { stack: Vec::new(), names: HashMap::new() }
    }

    pub fn interpret(&mut self, token: Token) {
        match token {
            Token::Number(_) => self.stack.push(token),
            Token::Print => self.print(),
            Token::Name(_) => self.stack.push(token),
            Token::Assign => self.assign(),
            Token::Add => self.add(),
            Token::Subtract => self.sub(),
        }
    }

    fn print(&mut self) {
        if self.stack.is_empty() {
            println!("Stack is empty.");
            return;
        }
        let value = self.get_value_from_stack();
        println!("Top stack value: {}", value);
    }

    fn add(&mut self) {
        let a = self.get_value_from_stack();
        let b = self.get_value_from_stack();
        self.stack.push(Token::Number(b + a));
    }

    fn sub(&mut self) {
        let a = self.get_value_from_stack();
        let b = self.get_value_from_stack();
        self.stack.push(Token::Number(b - a));
    }

    fn assign(&mut self) {
        let token1 = self.stack.pop();
        let name = match token1 {
            Some(Token::Name(name)) => name,
            _ => panic!("Syntax error!")
        };
        let value = self.get_value_from_stack();
        self.names.insert(name, value);
    }

    fn get_value_from_stack(&mut self) -> i64 {
        let token = self.stack.pop();
        match token {
            Some(Token::Number(n)) => n,
            Some(Token::Name(name)) => self.get_value(name),
            _ => panic!("Syntax error!")
        }
    }

    fn get_value(&self, name: String) -> i64 {
        match self.names.get(&name) {
            Some(n) => *n,
            None => panic!("Undefined variable: {}", name)
        }
    }

}

fn main() {
    let mut vm = VirtualMachine::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("flush() failed.");
        let tokens = parse_line();
        for token in tokens {
            vm.interpret(token);
        }
    }
}

fn parse_line() -> Vec<Token> {
    let stdin = io::stdin();
    let mut line = String::new();
    let parts = match stdin.read_line(&mut line) {
        Ok(_) => line.split_whitespace(),
        Err(_) => panic!("Suca fortissimo!")
    };
    let parts = parts.collect::<Vec<&str>>();
    let mut result = Vec::<Token>::with_capacity(parts.len());

    for part in parts {
        let part = part.trim();
        let token = match part.chars().next().unwrap() {
            '+' => Token::Add,
            '-' => Token::Subtract,
            '=' => Token::Assign,
            '@' => Token::Print,
            'a'..='z' => Token::Name(part.to_string()),
            '0'..='9' => Token::Number(part.parse::<i64>().unwrap()),
            _ => panic!("Error: unrecognized syntax.")
        };
        result.push(token);
    }
    result
}

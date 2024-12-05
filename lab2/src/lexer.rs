use regex::Regex;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use lazy_static::lazy_static;


lazy_static! {
    static ref KEYWORDS: Vec<&'static str> = vec![
        "позначити", "прямокутник", "побудувати", "визначити", "площу",
        "периметр", "перемістити", "змінити", "розмір", "повернути",
        "відзеркалити", "перетин", "точка", "висота", "ширина",
        "діагональ", "довжина", "координати", "центр"
    ];

    static ref MEASUREMENT_UNITS: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("m", 100.0);
        m.insert("м", 100.0);
        m.insert("метр", 100.0);
        m.insert("cm", 1.0);
        m.insert("см", 1.0);
        m.insert("сантиметр", 1.0);
        m.insert("mm", 0.1);
        m.insert("мм", 0.1);
        m.insert("міліметр", 0.1);
        m
    };
    static ref POINT_NAME_REGEX: Regex = Regex::new(r"^[A-Z][0-9]*$").unwrap();
    static ref RECTANGLE_NAME_REGEX: Regex = Regex::new(r"^([A-Z][0-9]*){4}$").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"^[+-]?\d+(\.\d+)?$").unwrap();
    static ref EXPRESSION_REGEX: Regex = Regex::new(r"\(?\-?\d+(\.\d+)?[\+\-\*\/]\d+(\.\d+)?\)?").unwrap();
}

pub struct Lexer {
    text: String,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            text: input.to_string(),
        }
    }
    fn split_with_delimiters_regex(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_word = String::new();
        let mut chars = self.text.chars().peekable();
        let mut paren_stack = Vec::new();
        let mut is_expression = false;

        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    if !current_word.is_empty() {
                        result.push(current_word.clone());
                        current_word.clear();
                    }

                    let mut look_ahead = String::new();
                    let mut temp_paren_level = 1;
                    let mut found_comma = false;
                    let mut peek_chars = chars.clone();
                    let mut found_close = false;

                    while let Some(next_c) = peek_chars.next() {
                        look_ahead.push(next_c);
                        match next_c {
                            '(' => temp_paren_level += 1,
                            ')' => {
                                temp_paren_level -= 1;
                                if temp_paren_level == 0 {
                                    found_close = true;
                                    break;
                                }
                            }
                            ',' => {
                                if temp_paren_level == 1 {
                                    found_comma = true;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    if found_comma || (found_close && look_ahead.split(',').count() == 2
                        && look_ahead.chars().all(|c| c.is_numeric() || c == ',' || c == ')' || c.is_whitespace())) {
                        result.push(c.to_string());
                    } else {
                        current_word.push(c);
                        paren_stack.push(true);
                        is_expression = true;
                    }
                }
                ')' => {
                    if is_expression {
                        current_word.push(c);
                        if let Some(_) = paren_stack.pop() {
                            if paren_stack.is_empty() {
                                result.push(current_word.clone());
                                current_word.clear();
                                is_expression = false;
                            }
                        }
                    } else {
                        if !current_word.is_empty() {
                            result.push(current_word.clone());
                            current_word.clear();
                        }
                        result.push(c.to_string());
                    }
                }
                ',' | ';' | '\'' | '.' | '?' | '!' | ':' | '"' => {
                    if !is_expression {
                        if !current_word.is_empty() {
                            result.push(current_word.clone());
                            current_word.clear();
                        }
                        result.push(c.to_string());
                    } else {
                        current_word.push(c);
                    }
                }
                ' ' => {
                    if !is_expression {
                        if !current_word.is_empty() {
                            result.push(current_word.clone());
                            current_word.clear();
                        }
                    } else {
                        current_word.push(c);
                    }
                }
                _ => {
                    current_word.push(c);
                }
            }
        }

        if !current_word.is_empty() {
            result.push(current_word);
        }

        result
    }

    pub fn process(&mut self) -> String {
        let words: Vec<String> = self.split_with_delimiters_regex();
        let mut result = Vec::new();
        let mut i = 0;

        //println!("word: {:?}", words);
        while i < words.len() {
            let word = &words[i];

            if word.trim().is_empty() {
                i += 1;
                continue;
            }
            if RECTANGLE_NAME_REGEX.is_match(word) {
                result.push(word.clone());
            } else if POINT_NAME_REGEX.is_match(word) {
                result.push(word.clone());
            } else if let Some(processed) = self.process_measurement(word, words.get(i + 1).map(|s| s.as_str())) {
                result.push(processed);
                if words.get(i + 1).map_or(false, |next| self.has_unit(Some(next))) {
                    i += 1;
                }
            } else if let Some(expr) = self.try_parse_expression(word) {
                result.push(expr);
            } else {
                result.push(word.to_lowercase());
            }
            i += 1;
        }

        result.join(" ")
    }

    fn try_parse_expression(&self, word: &str) -> Option<String> {
        if !word.contains(|c: char| ['+', '-', '*', '/', '(', ')'].contains(&c)) {
            return None;
        }

        if word.chars().count() == 1 && (word == "(" || word == ")") {
            return Some(word.to_string());
        }

        let mut evaluator = ExpressionEvaluator::new(word);
        match evaluator.evaluate() {
            Ok(result) => Some(result.to_string()),
            Err(_) => None,
        }
    }

    fn process_measurement(&self, number: &str, next_word: Option<&str>) -> Option<String> {
        if !NUMBER_REGEX.is_match(number) {
            return None;
        }

        let value = number.parse::<f64>().ok()?;

        if let Some(unit) = next_word {
            for (unit_name, coefficient) in MEASUREMENT_UNITS.iter() {
                if (unit.chars().count() > 3 && unit.starts_with(unit_name))
                    || (unit.chars().count() < 3 && &unit == unit_name){
                    return Some((value * coefficient).to_string());
                }
            }
        }

        Some(value.to_string())
    }

    fn has_unit(&self, word: Option<&str>) -> bool {
        if let Some(w) = word {
            for unit in MEASUREMENT_UNITS.keys() {
                if w.starts_with(unit) {
                    return true;
                }
            }
        }
        false
    }
}
#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
struct ExpressionEvaluator {
    input: String,
}

impl ExpressionEvaluator {
    fn new(input: &str) -> Self {
        ExpressionEvaluator {
            input: input.replace(" ", ""),
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = self.input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    tokens.push(Token::Number(self.parse_number(&mut chars)?));
                }
                '+' => {
                    chars.next();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    chars.next();
                    if tokens.is_empty() || matches!(tokens.last(), Some(Token::LeftParen)) {
                        if let Some(&next_char) = chars.peek() {
                            if next_char.is_digit(10) {
                                let num = self.parse_number(&mut chars)?;
                                tokens.push(Token::Number(-num));
                                continue;
                            }
                        }
                    }
                    tokens.push(Token::Minus);
                }
                '*' => {
                    chars.next();
                    tokens.push(Token::Multiply);
                }
                '/' => {
                    chars.next();
                    tokens.push(Token::Divide);
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RightParen);
                }
                ' ' => {
                    chars.next();
                }
                _ => return Err(format!("Неочікуваний символ: {}", c)),
            }
        }
        Ok(tokens)
    }

    fn parse_number(&self, chars: &mut Peekable<Chars>) -> Result<f64, String> {
        let mut number_str = String::new();

        while let Some(&c) = chars.peek() {
            if c.is_digit(10) || c == '.' {
                number_str.push(c);
                chars.next();
            } else {
                break;
            }
        }

        number_str.parse::<f64>()
            .map_err(|_| "Parsing error.".to_string())
    }

    fn evaluate(&mut self) -> Result<f64, String> {
        let tokens = self.tokenize()?;
        self.evaluate_expression(&tokens, 0).map(|(result, _)| result)
    }

    fn evaluate_expression(&self, tokens: &[Token], pos: usize) -> Result<(f64, usize), String> {
        let mut result = 0.0;
        let mut current_pos = pos;
        let mut operation = Token::Plus;

        while current_pos < tokens.len() {
            let (value, new_pos) = match &tokens[current_pos] {
                Token::Number(n) => (*n, current_pos + 1),
                Token::LeftParen => {
                    let (sub_result, new_pos) = self.evaluate_expression(tokens, current_pos + 1)?;
                    (sub_result, new_pos)
                }
                Token::RightParen => return Ok((result, current_pos + 1)),
                _ => {
                    operation = tokens[current_pos].clone();
                    current_pos += 1;
                    continue;
                }
            };

            result = match operation {
                Token::Plus => result + value,
                Token::Minus => result - value,
                Token::Multiply => result * value,
                Token::Divide => {
                    if value == 0.0 {
                        return Err("Null division error.".to_string());
                    }
                    result / value
                }
                _ => result,
            };

            current_pos = new_pos;
        }

        Ok((result, current_pos))
    }
}

use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::TokenType::{Arithmetic, Assignment, Comparison, Delimiter, Identifier, Invalid, Logical, StringLiteral, CharLiteral};

const KEY_WORDS: [&str; 13] = ["if", "else", "let", "for", "loop", "while", "mut", "enum", "struct", "fn", "impl", "use", "macro"];
const IDENTIFIER_SYMBOLS: &str = "^[shvachkaSHVACHKAшвачкаШВАЧКА0-9_]+$";
const FUNCTIONS: [&str; 6] = ["sin", "cos", "tan", "ctan", "sqrt", "new"];
const COMPARISON_SYMBOLS: [&str; 6] = ["<", "<=", ">=", ">", "==", "!="];
const ARITHMETIC_SYMBOLS: [&str; 5] = ["-", "+", "*", "/", "%"];
const DELIMITER_SYMBOLS: [&str; 14] = [";", ":", "::", ",", "[", "]", "(", ")", "{", "}", "?", "\'", "\"", "&"];
const LOGICAL_SYMBOLS: [&str; 3] = ["&&", "||", "!"];
const ASSIGNMENT_SYMBOLS: [&str; 6] = ["=", "/=", "*=", "+=", "-=", "%="];
const DATA_TYPES: [&str; 18] = ["char", "String", "str", "bool", "f32", "f64", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize"];

const STRING_START: &str = "\"";
const CHAR_START: &str = "'";

#[derive(PartialEq, Eq)]
#[derive(Debug)]
enum TokenType {
    DataType,
    Integer,
    Double,
    Identifier,
    Arithmetic,
    Function,
    KeyWord,
    Delimiter,
    Comparison,
    Logical,
    Assignment,
    Boolean,
    StringLiteral,
    CharLiteral,
    Invalid,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Token {
    type_name: TokenType,
    content: String,
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let input = File::open(file_path)?;
    let buffered = BufReader::new(input);
    let text = buffered.lines().collect::<Result<Vec<_>, _>>()?.join("");

    Ok(text)
}

fn escape_special_chars(input: &str) -> String {
    let mut escaped = String::new();
    for c in input.chars() {
        if r"\^$.*+?()[]{}|".contains(c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

fn generate_regex(expr: &[&str]) -> Result<Regex, Box<dyn Error>> {
    let escaped_expr: Vec<String> = expr.iter().map(|s| escape_special_chars(s)).collect();
    let content = format!("^({})$", escaped_expr.join("|"));
    //println!("{:?}", content);
    Ok(Regex::new(&*content).unwrap())
}

fn get_string_of_char(p: &usize, input: &String) -> Result<String, Box<dyn Error>> {
    if p < &input.chars().count() {
        if let Some(c) = input.chars().nth(*p) {
            return Ok(c.to_string());
        }
    }
    Err("Index out of bounds".into())
}

fn match_token_type(symbol: &str, comparison_regex: &Regex, delimiter_regex: &Regex, logical_regex: &Regex, assign_regex: &Regex) -> TokenType {
    if comparison_regex.is_match(symbol) {
        Comparison
    } else if delimiter_regex.is_match(symbol) {
        Delimiter
    } else if logical_regex.is_match(symbol) {
        Logical
    } else if assign_regex.is_match(symbol) {
        Assignment
    } else {
        Invalid
    }
}

fn lex(file_path: &str) -> Result<(Vec<Token>, Vec<Token>), Box<dyn std::error::Error>> {
    let input = read_file(file_path)?;
    let mut p = 0;
    let mut tokens: Vec<Token> = Vec::new();
    let mut identifiers: Vec<Token> = Vec::new();

    let identifier_regex = Regex::new(IDENTIFIER_SYMBOLS)?;
    let whitespace_regex = Regex::new("\\s")?;
    let number_regex = Regex::new("[0-9]+")?;
    let literal_start_regex = Regex::new("[a-zA-Zа-яА-ЯіІїЇєЄґҐ]+")?;
    let literal_regex = Regex::new("[a-zA-Zа-яА-ЯіІїЇєЄґҐ0-9_]+")?;
    let comparison_regex = generate_regex(&COMPARISON_SYMBOLS)?;
    let assignment_regex = generate_regex(&ASSIGNMENT_SYMBOLS)?;
    let arithmetic_regex = generate_regex(&ARITHMETIC_SYMBOLS)?;
    let delimiter_regex = generate_regex(&DELIMITER_SYMBOLS)?;
    let logical_regex = generate_regex(&LOGICAL_SYMBOLS)?;
    let function_regex = generate_regex(&FUNCTIONS)?;
    let key_words_regex = generate_regex(&KEY_WORDS)?;
    let data_types_regex = generate_regex(&DATA_TYPES)?;

    while p < input.chars().count() {
        let current: &str = &*get_string_of_char(&p, &input)?;
        if whitespace_regex.is_match(current) {
            p += 1;
            continue;
        }
        if current == STRING_START {
            let token = get_string_literal_token(&mut p, &input)?;
            tokens.push(token);
            continue;
        }
        if current == CHAR_START {
            let token = get_char_literal_token(&mut p, &input)?;
            tokens.push(token);
            continue;
        }
        if literal_start_regex.is_match(&current) {
            let token = get_literal_token(&mut p, &input, &literal_regex, &key_words_regex, &identifier_regex, &function_regex, &data_types_regex)?;
            if token.type_name == Identifier {
                if !identifiers.iter().any(|id| id.content == token.content) {
                    identifiers.push(Token { type_name: Identifier, content: token.content.clone() });
                }
            }
            tokens.push(token);
            continue;
        }
        if arithmetic_regex.is_match(current) {
            let t = get_operator_or_numeric_token(&mut p, &input, &number_regex, &arithmetic_regex, &assignment_regex)?;
            tokens.push(t);
            continue;
        }
        if number_regex.is_match(current) {
            let t = get_numeric_token(&mut p, &input, &number_regex)?;
            tokens.push(t);
            continue;
        }
        let token = get_complex_token(&mut p, &input, &comparison_regex, &delimiter_regex, &logical_regex, &assignment_regex)?;
        tokens.push(token);
    }

    Ok((tokens, identifiers))
}

fn get_complex_token(p: &mut usize, input: &String, comparison_regex: &Regex, delimiter_regex: &Regex, logical_regex: &Regex, assign_regex: &Regex) -> Result<Token, Box<dyn Error>> {
    let current = get_string_of_char(p, input)?;
    let is_not_the_end = *p + 1 < input.chars().count();
    let next_char = if is_not_the_end { get_string_of_char(&(*p + 1), input)? } else { String::new() };

    let combined = current.clone() + &next_char;

    if is_not_the_end && (comparison_regex.is_match(&combined) || delimiter_regex.is_match(&combined) || logical_regex.is_match(&combined)) {
        *p += 2;
        return Ok(Token { type_name: match_token_type(&combined, comparison_regex, delimiter_regex, logical_regex, assign_regex), content: combined });
    }
    if comparison_regex.is_match(&current) || delimiter_regex.is_match(&current) || assign_regex.is_match(&current) {
        *p += 1;
        return Ok(Token { type_name: match_token_type(&current, comparison_regex, delimiter_regex, logical_regex, assign_regex), content: current });
    }

    *p += 1;
    Ok(Token { type_name: Invalid, content: current })
}

fn get_string_literal_token(p: &mut usize, input: &String) -> Result<Token, Box<dyn Error>> {
    *p += 1;
    let mut string_content = String::from(STRING_START);

    while *p < input.chars().count() {
        let current_char = get_string_of_char(p, input)?;

        if current_char == STRING_START {
            string_content.push_str(STRING_START);
            *p += 1;
            return Ok(Token {
                type_name: StringLiteral,
                content: string_content,
            });
        }

        string_content.push_str(&current_char);
        *p += 1;
    }

    //Err("Endless string".into())
    Ok(Token { type_name: Invalid, content: string_content })
}

fn get_char_literal_token(p: &mut usize, input: &String) -> Result<Token, Box<dyn Error>> {
    *p += 1;
    let mut char_content = String::from(CHAR_START);

    if *p+1 < input.chars().count() {
        if get_string_of_char(&(*p), input)? != CHAR_START
            && get_string_of_char(&(*p + 1), input)? == CHAR_START {
            char_content.push_str(&*get_string_of_char(p, input)?);
            char_content.push_str(CHAR_START);
            *p += 2;
            return Ok(Token {type_name: CharLiteral, content: char_content});
        }
    }
    Ok(Token { type_name: Delimiter, content: char_content })
}

fn get_operator_or_numeric_token(p: &mut usize, input: &String, num_regex: &Regex, arith_regex: &Regex, assign_regex: &Regex) -> Result<Token, Box<dyn Error>> {
    let prev_symbol = if *p > 0 { get_string_of_char(&(*p-1), input)? } else { String::new() };
    let symbol = get_string_of_char(p, input)?;
    let mut operator = symbol.clone();

    if arith_regex.is_match(&symbol) {
        let plus_or_minus = symbol == "+" || symbol == "-";
        *p += 1;
        if *p < input.chars().count() {
            let current_symbol = get_string_of_char(p, input)?;
            if plus_or_minus && num_regex.is_match(&current_symbol)
                && !num_regex.is_match(&prev_symbol){
                let token = get_numeric_token(p, input, num_regex)?;
                operator.push_str(&token.content);
                return Ok(Token { type_name: token.type_name, content: operator });
            }
            if assign_regex.is_match(&(operator.clone() + &current_symbol)) {
                operator.push_str(&current_symbol);
                *p += 1;
                return Ok(Token { type_name: Assignment, content: operator });
            }
            return Ok(Token { type_name: Arithmetic, content: operator });
        }
    }

    Ok(Token {type_name: Invalid, content: operator})
}

fn get_literal_token(p: &mut usize, input: &String, literal_regex: &Regex, key_word_regex: &Regex, identifier_regex: &Regex, function_regex: &Regex, data_type_regex: &Regex) -> Result<Token, Box<dyn Error>> {
    let mut literal: String = String::new();
    while *p < input.chars().count() && literal_regex.is_match(&*get_string_of_char(&p, &input)?) {
        literal.push_str(&*get_string_of_char(&p, &input)?);
        *p += 1;
    }

    if literal == "true" || literal == "false" {
        return Ok(
            Token {
                type_name: TokenType::Boolean,
                content: literal,
            }
        );
    }

    if key_word_regex.is_match(&*literal) {
        return Ok(
            Token {
                type_name: TokenType::KeyWord,
                content: literal,
            }
        );
    }

    if data_type_regex.is_match(&*literal) {
        return Ok(
            Token {
                type_name: TokenType::DataType,
                content: literal,
            }
        );
    }

    if function_regex.is_match(&*literal) {
        return Ok(Token { type_name: TokenType::Function, content: literal });
    }

    if identifier_regex.is_match(&*literal) {
        return Ok(
            Token {
                type_name: Identifier,
                content: literal,
            }
        );
    }

    println!("Invalid literal token: {}", literal);
    Ok(
        Token {
            type_name: Invalid,
            content: literal,
        }
    )
}

fn get_numeric_token(p: &mut usize, input: &String, regex: &Regex) -> Result<Token, Box<dyn Error>> {
    let mut is_float = false;
    let mut number = String::new();
    while *p < input.chars().count() && (regex.is_match(&*get_string_of_char(&p, &input)?) || &*get_string_of_char(&p, &input)? == ".") {
        let char = get_string_of_char(&p, &input)?;
        if &*char == "." {
            is_float = true;
        }
        number.push_str(&*char);
        *p += 1;
    }

    Ok(
        Token {
            type_name: if is_float {
                TokenType::Double
            } else { TokenType::Integer },
            content: number,
        }
    )
}


fn main() {
    let path = "data/test.txt";
    match lex(path) {
        Ok(data) => {
            println!("Success!\n\nTokens:\n");

            for token in &data.0 {
                println!("{} ---> {}", token.content, token.type_name.to_string());
            }
            println!("\nIdentifiers:\n\n");
            for token in &data.1 {
                println!("{} ---> {}", token.content, token.type_name.to_string());
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

use regex::Regex;
use std::collections::HashMap;

pub fn preprocess(input: &str) -> String {
    let mut units = vec![
        ("км²", (10000000000.0, "см²")),
        ("дм²", (100.0, "см²")),
        ("мм²", (0.01, "см²")),
        ("м²", (10000.0, "см²")),
        ("км", (100000.0, "см")),
        ("дм", (10.0, "см")),
        ("мм", (0.1, "см")),
        ("м", (100.0, "см")),
    ];

    units.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let mut result = input.to_lowercase();

    for (unit, (multiplier, target_unit)) in units {
        let pattern = format!(r"(\d+(?:\.\d+)?)\s*{}", regex::escape(unit));
        let re_unit = Regex::new(&pattern).unwrap();

        result = re_unit.replace_all(&result, |caps: &regex::Captures| {
            let number: f64 = caps[1].parse().unwrap();
            let converted = number * multiplier;
            format!("{:.1} {}", converted, target_unit)
        }).to_string();
    }

    let re_decimal = Regex::new(r"(\d+)\.(\d+)").unwrap();
    result = re_decimal.replace_all(&result, "${1}_${2}").to_string();
    move_condition_to_start(&result)
}

pub fn restore_dots(tokens: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"(\d+)_(\d+)").unwrap();

    tokens
        .iter()
        .map(|token| {
            if re.is_match(token) {
                re.replace_all(token, "${1}.${2}").to_string()
            } else {
                token.clone()
            }
        })
        .collect()
}

fn move_condition_to_start(text: &str) -> String {
    let re = Regex::new(r"(якщо[^.?!]*[.?!]?)").unwrap();
    if let Some(captures) = re.captures(text) {
        if let Some(condition) = captures.get(0) {
            let condition = condition.as_str();
            let remaining_text = text.replacen(condition, "", 1);
            return format!("{}, {}", condition.trim(), remaining_text.trim());
        }
    }
    text.to_string()
}

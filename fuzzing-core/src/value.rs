// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

use crate::generator::{BitSeg, Expr, Module, Stmt};
use gleam_core::build::Target;
use std::sync::LazyLock;

/// ANSI escape regex from chalk/ansi-regex v6.0.1 (MIT).
/// Matches OSC, CSI, and other ANSI escape sequences.
/// <https://github.com/chalk/ansi-regex/blob/main/index.js>
static ANSI_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?:\x07|\x1b\\|\x9c)|(?:\x1b\][^\x07\x1b\x9c]*(?:\x07|\x1b\\|\x9c))|[\x1b\x9b][\[\]()*#;?]*(?:\d{1,4}(?:[;:]\d{0,4})*)?[\dA-PR-TZcf-nq-uy=><~]",
    )
    .unwrap()
});

/// A Gleam value as printed by `echo`.
///
/// Erlang and JavaScript render values differently. For example,
/// Erlang prints `<<1, 2>>` as `"\u{0001}\u{0002}"` while JavaScript
/// prints `<<1, 2>>`. Both parse to the same `Value`, so comparison is exact.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    BitArray(Vec<u8>),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Constructor(String, Vec<Value>),
}

/// Convert a generator `Expr` to a `Value`.
///
/// Handles literal expressions only. The generator echoes literals,
/// not computed expressions.
///
pub fn expected_value(expr: &Expr) -> Option<Value> {
    match expr {
        Expr::IntLit(n) => Some(Value::Int(*n)),
        Expr::FloatLit(f) => Some(Value::Float(*f)),
        Expr::StrLit(s) => Some(Value::String(s.to_string())),
        Expr::BoolLit(b) => Some(Value::Bool(*b)),
        Expr::ListLit(xs) => {
            let vals: Option<Vec<Value>> = xs.iter().map(expected_value).collect();
            Some(Value::List(vals?))
        }
        Expr::TupLit(a, b) => {
            let va = expected_value(a)?;
            let vb = expected_value(b)?;
            Some(Value::Tuple(vec![va, vb]))
        }
        Expr::BitsLit(segs) => {
            let mut bytes = Vec::new();
            for seg in segs {
                match seg {
                    BitSeg::Int(val, _width) => {
                        bytes.push(*val as u8);
                    }
                    BitSeg::Utf8(s) => {
                        bytes.extend_from_slice(s.as_bytes());
                    }
                }
            }
            Some(Value::BitArray(bytes))
        }
        Expr::NegInt(e) => {
            let v = expected_value(e)?;
            match v {
                Value::Int(n) => Some(Value::Int(-n)),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn expected_output(module: &Module) -> Vec<Value> {
    module
        .main_stmts()
        .iter()
        .filter_map(|stmt| match stmt {
            Stmt::Echo(expr) => expected_value(expr),
            _ => None,
        })
        .collect()
}

pub fn parse_value(line: &str, target: Target) -> Option<Value> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    match target {
        // The native target deliberately mirrors Erlang's `echo`
        // rendering, so its output parses with the Erlang parser.
        Target::Erlang | Target::Native => parse_erlang(line),
        Target::JavaScript => parse_javascript(line),
    }
}

/// Find the 1-indexed line numbers of every `echo` statement in the source.
///
/// Each Gleam `echo` prints `<file>:<line>` followed by the value. A value
/// follows only a marker whose line number is in the returned set, so
/// compiler warning text is not picked up as a value.
///
pub fn echo_line_numbers(source: &str) -> std::collections::HashSet<u32> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with("echo "))
        .map(|(i, _)| (i + 1) as u32)
        .collect()
}

/// Parse runtime output using the source's echo line numbers as markers.
///
/// Each `echo` runtime call writes a `<file>:<line>` marker followed by the
/// inspected value. Markers whose line number is not in `echo_lines` do
/// not start a value, so warning hint text is not mistaken for one.
///
pub fn parse_output_with_echo_lines(
    raw: &str,
    target: Target,
    echo_lines: &std::collections::HashSet<u32>,
) -> Vec<Value> {
    // Matches a line that is just `<file>:<number>` after ANSI stripping,
    // for example `src/main.gleam:11`.
    let echo_marker_re = regex::Regex::new(r"^[^\s:]+(?:/[^\s:]+)*:\d+$").unwrap();
    let line_num_re = regex::Regex::new(r":(\d+)$").unwrap();

    let mut values = Vec::new();
    let mut next_is_value = false;

    for line in raw.lines() {
        let stripped = ANSI_RE.replace_all(line, "");
        let trimmed = stripped.trim();

        if next_is_value {
            next_is_value = false;
            if !trimmed.is_empty() {
                if let Some(v) = parse_value(trimmed, target) {
                    values.push(v);
                }
            }
            continue;
        }

        if echo_marker_re.is_match(trimmed) {
            if let Some(caps) = line_num_re.captures(trimmed) {
                if let Ok(n) = caps[1].parse::<u32>() {
                    if echo_lines.contains(&n) {
                        next_is_value = true;
                        continue;
                    }
                }
            }
        }
    }
    values
}

/// Strip build noise from raw output.
/// Removes ANSI codes, warnings, source locations, and progress lines.
///
pub fn strip_build_noise(raw: &str) -> String {
    let source_loc_re = regex::Regex::new(r"\.(gleam|erl):\d+").unwrap();
    let progress_re = regex::Regex::new(
        r"^(Resolving versions|Compiling |Compiled in |Running |Downloading |Added |Downloaded )",
    )
    .unwrap();
    let hint_re = regex::Regex::new(r"^(Hint:|warning:|error:)").unwrap();
    let warning_context_re = regex::Regex::new(
        r"^(This segment|be truncated|resulting in|The |^It |\^|This pattern|Matching on|unreachable|redundant|This comparison)",
    )
    .unwrap();

    let mut result = String::new();
    // A warning block runs from `warning: ...` up to the next `Hint:`, a new
    // `warning:`, `Running`/`Compiled in`, or end of input. Blank lines
    // inside the block are part of it, so multi-paragraph hint text is
    // skipped too.
    let mut in_warning = false;

    for line in raw.lines() {
        let stripped = ANSI_RE.replace_all(line, "");
        let trimmed = stripped.trim();

        // A warning block ends at the next `warning:`, `Hint:`, or program
        // output line such as `Running` or `Compiled in`.
        if in_warning
            && (trimmed.starts_with("warning:")
                || progress_re.is_match(trimmed)
                || hint_re.is_match(trimmed))
        {
            in_warning = false;
        }

        if trimmed.starts_with("warning:") {
            in_warning = true;
            continue;
        }

        // Skip every line until a marker resets the flag.
        if in_warning {
            continue;
        }

        // Skip indented source-snippet lines after a warning.
        if warning_context_re.is_match(trimmed) {
            continue;
        }

        // Skip source location lines
        if source_loc_re.is_match(trimmed) {
            continue;
        }

        // Skip progress and hint lines
        if progress_re.is_match(trimmed) || hint_re.is_match(trimmed) {
            continue;
        }

        // Skip lines that look like compiler error context (indented, no value shape)
        if trimmed.starts_with('|') || trimmed.starts_with("^") || trimmed.starts_with("--") {
            continue;
        }

        result.push_str(trimmed);
        result.push('\n');
    }
    result
}

// ---------------------------------------------------------------------------
// Erlang parser
//
// Erlang echo@inspect renders:
//   nil       -> "Nil"
//   true      -> "True"
//   false     -> "False"
//   int       -> integer literal (e.g. "42", "-1")
//   float     -> float literal (e.g. "1.0", "-3.14", "1.0e6")
//   binary    -> "string" (UTF-8) or <<bytes>> (non-UTF-8)
//   bitarray  -> "string" (valid UTF-8) or <<bytes>> (non-UTF-8)
//   atom      -> Constructor
//   list      -> [elements]
//   tuple     -> #(elements)
//   record    -> Constructor(fields)
// ---------------------------------------------------------------------------

fn parse_erlang(line: &str) -> Option<Value> {
    let line = line.trim();
    match line {
        "Nil" => return Some(Value::Nil),
        "True" => return Some(Value::Bool(true)),
        "False" => return Some(Value::Bool(false)),
        _ => {}
    }

    // Integer
    if let Ok(n) = line.parse::<i64>() {
        return Some(Value::Int(n));
    }

    // Float
    if let Ok(f) = parse_erlang_float(line) {
        return Some(Value::Float(f));
    }

    // String (starts and ends with ")
    if line.starts_with('"') && line.ends_with('"') {
        let inner = &line[1..line.len() - 1];
        return Some(Value::String(unescape_erlang_string(inner)));
    }

    // Bit array: <<...>>
    if line.starts_with("<<") && line.ends_with(">>") {
        let inner = &line[2..line.len() - 2];
        let bytes = parse_erlang_bit_array(inner);
        return Some(Value::BitArray(bytes));
    }

    // List: [...]
    if line.starts_with('[') && line.ends_with(']') {
        let inner = &line[1..line.len() - 1];
        let items = split_erlang_items(inner);
        let vals: Vec<Value> = items.iter().filter_map(|s| parse_erlang(s)).collect();
        return Some(Value::List(vals));
    }

    // Tuple: #(...)
    if line.starts_with("#(") && line.ends_with(')') {
        let inner = &line[2..line.len() - 1];
        let items = split_erlang_items(inner);
        let vals: Vec<Value> = items.iter().filter_map(|s| parse_erlang(s)).collect();
        return Some(Value::Tuple(vals));
    }

    // Constructor: Name or Name(args)
    if let Some(v) = parse_erlang_constructor(line) {
        return Some(v);
    }

    None
}

fn parse_erlang_float(line: &str) -> Result<f64, ()> {
    // Handle scientific notation: 1.0e6, -3.14e2
    if line.contains('e') || line.contains('E') {
        line.parse::<f64>().map_err(|_| ())
    } else if line.contains('.') {
        line.parse::<f64>().map_err(|_| ())
    } else {
        Err(())
    }
}

fn unescape_erlang_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('u') => {
                    // \u{XXXX}
                    if chars.peek() == Some(&'{') {
                        chars.next(); // consume '{'
                        let hex: String = chars.by_ref().take_while(|c| *c != '}').collect();
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(ch) = char::from_u32(code) {
                                result.push(ch);
                            }
                        }
                    }
                }
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => {}
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn parse_erlang_bit_array(inner: &str) -> Vec<u8> {
    if inner.is_empty() {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    for part in inner.split(", ") {
        let part = part.trim();
        // Handle "value:size(N)" segments
        if let Some(pos) = part.find(":size(") {
            let val_str = &part[..pos];
            if let Ok(val) = val_str.parse::<u8>() {
                bytes.push(val);
            }
        } else if let Ok(val) = part.parse::<u8>() {
            bytes.push(val);
        }
    }
    bytes
}

fn split_erlang_items(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut items = Vec::new();
    let mut depth = 0;
    let mut current = String::new();
    let mut in_string = false;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if !in_string => {
                in_string = true;
                current.push(c);
            }
            '"' if in_string => {
                in_string = false;
                current.push(c);
            }
            '\\' if in_string => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            _ if in_string => {
                current.push(c);
            }
            '(' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(c);
            }
            '#' => {
                // # is part of tuple syntax #( but doesn't increment depth
                current.push(c);
            }
            ',' if depth == 0 => {
                items.push(current.trim().to_string());
                current = String::new();
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.trim().is_empty() {
        items.push(current.trim().to_string());
    }
    items
}

fn parse_erlang_constructor(line: &str) -> Option<Value> {
    // Constructor with args: Name(arg1, arg2, ...)
    if let Some(paren_pos) = line.find('(') {
        if line.ends_with(')') {
            let name = line[..paren_pos].to_string();
            let inner = &line[paren_pos + 1..line.len() - 1];
            let items = split_erlang_items(inner);
            let vals: Vec<Value> = items.iter().filter_map(|s| parse_erlang(s)).collect();
            return Some(Value::Constructor(name, vals));
        }
    }
    // Simple constructor: Name
    let name = line.to_string();
    if name.chars().next().map_or(false, |c| c.is_uppercase()) {
        return Some(Value::Constructor(name, Vec::new()));
    }
    None
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// JavaScript parser
//
// JavaScript echo@inspect renders:
//   undefined -> "Nil"
//   true      -> "True"
//   false     -> "False"
//   int       -> integer literal
//   float     -> float literal (e.g. "1", "3.14", "1e6")
//   string    -> "string"
//   bitarray  -> <<bytes>>
//   array     -> #(elements) (JS arrays)
//   list      -> [elements] (Gleam lists)
//   record    -> Constructor(fields)
//   tuple     -> #(elements) (JS tuples)
// ---------------------------------------------------------------------------

fn parse_javascript(line: &str) -> Option<Value> {
    let line = line.trim();
    match line {
        "Nil" => return Some(Value::Nil),
        "True" => return Some(Value::Bool(true)),
        "False" => return Some(Value::Bool(false)),
        _ => {}
    }

    // Integer
    if let Ok(n) = line.parse::<i64>() {
        return Some(Value::Int(n));
    }

    // Float (JavaScript doesn't print .0 for whole numbers)
    if let Ok(f) = line.parse::<f64>() {
        return Some(Value::Float(f));
    }

    // String
    if line.starts_with('"') && line.ends_with('"') {
        let inner = &line[1..line.len() - 1];
        return Some(Value::String(unescape_javascript_string(inner)));
    }

    // Bit array: <<...>>
    if line.starts_with("<<") && line.ends_with(">>") {
        let inner = &line[2..line.len() - 2];
        let bytes = parse_javascript_bit_array(inner);
        return Some(Value::BitArray(bytes));
    }

    // Tuple: #(elements)
    if line.starts_with("#(") && line.ends_with(')') {
        let inner = &line[2..line.len() - 1];
        let items = split_javascript_items(inner);
        let vals: Vec<Value> = items.iter().filter_map(|s| parse_javascript(s)).collect();
        return Some(Value::Tuple(vals));
    }

    // List: [elements]
    if line.starts_with('[') && line.ends_with(']') {
        let inner = &line[1..line.len() - 1];
        let items = split_javascript_items(inner);
        let vals: Vec<Value> = items.iter().filter_map(|s| parse_javascript(s)).collect();
        return Some(Value::List(vals));
    }

    // Constructor: Name or Name(args)
    if let Some(v) = parse_javascript_constructor(line) {
        return Some(v);
    }

    None
}

fn unescape_javascript_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('u') => {
                    // \u{XXXX}
                    let hex: String = chars.by_ref().take_while(|c| *c != '}').collect();
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = char::from_u32(code) {
                            result.push(ch);
                        }
                    }
                }
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => {}
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn parse_javascript_bit_array(inner: &str) -> Vec<u8> {
    if inner.is_empty() {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    for part in inner.split(", ") {
        let part = part.trim();
        // Handle "value:size(N)" segments
        if let Some(pos) = part.find(":size(") {
            let val_str = &part[..pos];
            if let Ok(val) = val_str.parse::<u8>() {
                bytes.push(val);
            }
        } else if let Ok(val) = part.parse::<u8>() {
            bytes.push(val);
        }
    }
    bytes
}

fn split_javascript_items(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut items = Vec::new();
    let mut depth = 0;
    let mut current = String::new();
    let mut in_string = false;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if !in_string => {
                in_string = true;
                current.push(c);
            }
            '"' if in_string => {
                in_string = false;
                current.push(c);
            }
            '\\' if in_string => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            _ if in_string => {
                current.push(c);
            }
            '(' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(c);
            }
            '#' => {
                // # is part of tuple syntax #( but doesn't increment depth
                current.push(c);
            }
            ',' if depth == 0 => {
                items.push(current.trim().to_string());
                current = String::new();
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.trim().is_empty() {
        items.push(current.trim().to_string());
    }
    items
}

fn parse_javascript_constructor(line: &str) -> Option<Value> {
    // Constructor with args: Name(arg1, arg2, ...)
    if let Some(paren_pos) = line.find('(') {
        if line.ends_with(')') {
            let name = line[..paren_pos].to_string();
            let inner = &line[paren_pos + 1..line.len() - 1];
            let items = split_javascript_items(inner);
            let vals: Vec<Value> = items.iter().filter_map(|s| parse_javascript(s)).collect();
            return Some(Value::Constructor(name, vals));
        }
    }
    // Simple constructor: Name
    let name = line.to_string();
    if name.chars().next().map_or(false, |c| c.is_uppercase()) {
        return Some(Value::Constructor(name, Vec::new()));
    }
    None
}

/// Compare two values accounting for known cross-target representation
/// differences (Float/Int, String/BitArray).
pub fn matches_cross_target(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Float(f), Value::Int(n)) | (Value::Int(n), Value::Float(f)) => {
            *f == *n as f64 && *f == (*n as f64).trunc()
        }
        (Value::String(s), Value::BitArray(bytes)) | (Value::BitArray(bytes), Value::String(s)) => {
            s.as_bytes() == bytes.as_slice()
        }
        (Value::List(a), Value::List(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| matches_cross_target(x, y))
        }
        (Value::Tuple(a), Value::Tuple(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| matches_cross_target(x, y))
        }
        (Value::Constructor(na, aa), Value::Constructor(nb, ab)) => {
            na == nb
                && aa.len() == ab.len()
                && aa
                    .iter()
                    .zip(ab.iter())
                    .all(|(x, y)| matches_cross_target(x, y))
        }
        _ => a == b,
    }
}

pub fn outputs_match_cross_target(a: &[Value], b: &[Value]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b.iter())
            .all(|(x, y)| matches_cross_target(x, y))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ----- expected_value tests -----

    #[test]
    fn expected_int() {
        let expr = Expr::IntLit(42);
        assert_eq!(expected_value(&expr), Some(Value::Int(42)));
    }

    #[test]
    fn expected_negative_int() {
        let expr = Expr::NegInt(Box::new(Expr::IntLit(42)));
        assert_eq!(expected_value(&expr), Some(Value::Int(-42)));
    }

    #[test]
    fn expected_float() {
        let expr = Expr::FloatLit(3.14);
        assert_eq!(expected_value(&expr), Some(Value::Float(3.14)));
    }

    #[test]
    fn expected_string() {
        let expr = Expr::StrLit("hello");
        assert_eq!(expected_value(&expr), Some(Value::String("hello".into())));
    }

    #[test]
    fn expected_bool() {
        assert_eq!(
            expected_value(&Expr::BoolLit(true)),
            Some(Value::Bool(true))
        );
        assert_eq!(
            expected_value(&Expr::BoolLit(false)),
            Some(Value::Bool(false))
        );
    }

    #[test]
    fn expected_list() {
        let expr = Expr::ListLit(vec![Expr::IntLit(1), Expr::IntLit(2), Expr::IntLit(3)]);
        assert_eq!(
            expected_value(&expr),
            Some(Value::List(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3)
            ]))
        );
    }

    #[test]
    fn expected_tuple() {
        let expr = Expr::TupLit(Box::new(Expr::IntLit(1)), Box::new(Expr::StrLit("two")));
        assert_eq!(
            expected_value(&expr),
            Some(Value::Tuple(vec![
                Value::Int(1),
                Value::String("two".into())
            ]))
        );
    }

    #[test]
    fn expected_bit_array() {
        let expr = Expr::BitsLit(vec![
            BitSeg::Int(1, 8),
            BitSeg::Int(2, 8),
            BitSeg::Int(3, 8),
        ]);
        assert_eq!(expected_value(&expr), Some(Value::BitArray(vec![1, 2, 3])));
    }

    #[test]
    fn expected_bit_array_utf8() {
        let expr = Expr::BitsLit(vec![BitSeg::Utf8("hello")]);
        assert_eq!(
            expected_value(&expr),
            Some(Value::BitArray(vec![104, 101, 108, 108, 111]))
        );
    }

    #[test]
    fn expected_empty_bit_array() {
        let expr = Expr::BitsLit(vec![]);
        assert_eq!(expected_value(&expr), Some(Value::BitArray(vec![])));
    }

    // ----- Erlang parser tests -----

    #[test]
    fn parse_erlang_int() {
        assert_eq!(parse_erlang("42"), Some(Value::Int(42)));
        assert_eq!(parse_erlang("-1"), Some(Value::Int(-1)));
        assert_eq!(parse_erlang("0"), Some(Value::Int(0)));
    }

    #[test]
    fn parse_erlang_float() {
        assert_eq!(parse_erlang("1.0"), Some(Value::Float(1.0)));
        assert_eq!(parse_erlang("-3.14"), Some(Value::Float(-3.14)));
        assert_eq!(parse_erlang("0.0"), Some(Value::Float(0.0)));
        assert_eq!(parse_erlang("1.0e6"), Some(Value::Float(1_000_000.0)));
    }

    #[test]
    fn parse_erlang_string() {
        assert_eq!(
            parse_erlang("\"hello\""),
            Some(Value::String("hello".into()))
        );
        assert_eq!(parse_erlang("\"\""), Some(Value::String("".into())));
        assert_eq!(
            parse_erlang("\"line1\\nline2\""),
            Some(Value::String("line1\nline2".into()))
        );
    }

    #[test]
    fn parse_erlang_bool() {
        assert_eq!(parse_erlang("True"), Some(Value::Bool(true)));
        assert_eq!(parse_erlang("False"), Some(Value::Bool(false)));
    }

    #[test]
    fn parse_erlang_nil() {
        assert_eq!(parse_erlang("Nil"), Some(Value::Nil));
    }

    #[test]
    fn parse_erlang_list() {
        assert_eq!(
            parse_erlang("[1, 2, 3]"),
            Some(Value::List(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3)
            ]))
        );
        assert_eq!(parse_erlang("[]"), Some(Value::List(vec![])));
    }

    #[test]
    fn parse_erlang_tuple() {
        assert_eq!(
            parse_erlang("#(1, \"two\")"),
            Some(Value::Tuple(vec![
                Value::Int(1),
                Value::String("two".into())
            ]))
        );
    }

    #[test]
    fn parse_erlang_bit_array() {
        assert_eq!(
            parse_erlang("<<1, 2, 3>>"),
            Some(Value::BitArray(vec![1, 2, 3]))
        );
        assert_eq!(parse_erlang("<<>>"), Some(Value::BitArray(vec![])));
    }

    #[test]
    fn parse_erlang_utf8_string_as_bit_array() {
        assert_eq!(
            parse_erlang("\"\\u{0001}\\u{0002}\\u{0003}\""),
            Some(Value::String("\u{0001}\u{0002}\u{0003}".into()))
        );
    }

    #[test]
    fn parse_erlang_constructor() {
        assert_eq!(
            parse_erlang("Ok(1)"),
            Some(Value::Constructor("Ok".into(), vec![Value::Int(1)]))
        );
        assert_eq!(
            parse_erlang("Red"),
            Some(Value::Constructor("Red".into(), vec![]))
        );
    }

    #[test]
    fn parse_erlang_nested() {
        assert_eq!(
            parse_erlang("[Ok(1), Error(\"boom\")]"),
            Some(Value::List(vec![
                Value::Constructor("Ok".into(), vec![Value::Int(1)]),
                Value::Constructor("Error".into(), vec![Value::String("boom".into())]),
            ]))
        );
    }

    // ----- JavaScript parser tests -----

    #[test]
    fn parse_javascript_int() {
        assert_eq!(parse_javascript("42"), Some(Value::Int(42)));
        assert_eq!(parse_javascript("-1"), Some(Value::Int(-1)));
    }

    #[test]
    fn parse_javascript_float() {
        assert_eq!(parse_javascript("1"), Some(Value::Int(1))); // JS doesn't print .0
        assert_eq!(parse_javascript("3.14"), Some(Value::Float(3.14)));
        assert_eq!(parse_javascript("1e6"), Some(Value::Float(1_000_000.0)));
    }

    #[test]
    fn parse_javascript_string() {
        assert_eq!(
            parse_javascript("\"hello\""),
            Some(Value::String("hello".into()))
        );
    }

    #[test]
    fn parse_javascript_bool() {
        assert_eq!(parse_javascript("True"), Some(Value::Bool(true)));
        assert_eq!(parse_javascript("False"), Some(Value::Bool(false)));
    }

    #[test]
    fn parse_javascript_nil() {
        assert_eq!(parse_javascript("Nil"), Some(Value::Nil));
    }

    #[test]
    fn parse_javascript_bit_array() {
        assert_eq!(
            parse_javascript("<<1, 2, 3>>"),
            Some(Value::BitArray(vec![1, 2, 3]))
        );
        assert_eq!(parse_javascript("<<>>"), Some(Value::BitArray(vec![])));
    }

    #[test]
    fn parse_javascript_tuple() {
        assert_eq!(
            parse_javascript("#(1, \"two\")"),
            Some(Value::Tuple(vec![
                Value::Int(1),
                Value::String("two".into())
            ]))
        );
    }

    #[test]
    fn parse_javascript_constructor() {
        assert_eq!(
            parse_javascript("Ok(1)"),
            Some(Value::Constructor("Ok".into(), vec![Value::Int(1)]))
        );
        assert_eq!(
            parse_javascript("Red"),
            Some(Value::Constructor("Red".into(), vec![]))
        );
    }

    // ----- Cross-target equivalence tests -----
    // These verify that the same value parses to the same Value from both targets

    #[test]
    fn cross_target_int() {
        let erl = parse_erlang("42");
        let js = parse_javascript("42");
        assert_eq!(erl, js);
    }

    #[test]
    fn cross_target_bool() {
        let erl = parse_erlang("True");
        let js = parse_javascript("True");
        assert_eq!(erl, js);
    }

    #[test]
    fn cross_target_string() {
        let erl = parse_erlang("\"hello\"");
        let js = parse_javascript("\"hello\"");
        assert_eq!(erl, js);
    }

    #[test]
    fn cross_target_bit_array() {
        let erl = parse_erlang("<<1, 2, 3>>");
        let js = parse_javascript("<<1, 2, 3>>");
        assert_eq!(erl, js);
        assert_eq!(erl, Some(Value::BitArray(vec![1, 2, 3])));
    }

    #[test]
    fn cross_target_bit_array_from_utf8() {
        let erl = parse_erlang("\"\\u{0001}\\u{0002}\\u{0003}\"");
        let js = parse_javascript("<<1, 2, 3>>");
        assert_ne!(
            erl, js,
            "Erlang renders bit arrays as UTF-8 strings, JS as <<...>>"
        );
        // But they should match cross-target
        assert!(matches_cross_target(&erl.unwrap(), &js.unwrap()));
    }

    #[test]
    fn cross_target_float() {
        let erl = parse_erlang("1.0");
        let js = parse_javascript("1");
        assert_eq!(erl, Some(Value::Float(1.0)));
        assert_eq!(js, Some(Value::Int(1)));
        // They differ structurally but match cross-target
        assert!(matches_cross_target(&erl.unwrap(), &js.unwrap()));
    }

    #[test]
    fn cross_target_float_non_whole() {
        let erl = parse_erlang("1.5");
        let js = parse_javascript("1.5");
        assert_eq!(erl, Some(Value::Float(1.5)));
        assert_eq!(js, Some(Value::Float(1.5)));
        assert!(matches_cross_target(&erl.unwrap(), &js.unwrap()));
    }

    #[test]
    fn cross_target_float_zero_dot_zero() {
        let erl = parse_erlang("0.0");
        let js = parse_javascript("0");
        assert!(matches_cross_target(&erl.unwrap(), &js.unwrap()));
    }

    #[test]
    fn cross_target_list() {
        let erl = parse_erlang("[1, 2, 3]");
        let js = parse_javascript("[1, 2, 3]");
        assert_eq!(erl, js);
    }

    #[test]
    fn cross_target_constructor() {
        let erl = parse_erlang("Ok(1)");
        let js = parse_javascript("Ok(1)");
        assert_eq!(erl, js);
    }

    #[test]
    fn cross_target_list_with_float_int() {
        // This is the exact pattern from the fuzz divergence output
        let erl = parse_erlang("[\"ab\", 0.0, 1.0]");
        let js = parse_javascript("[\"ab\", 0, 1]");
        assert_eq!(
            erl,
            Some(Value::List(vec![
                Value::String("ab".into()),
                Value::Float(0.0),
                Value::Float(1.0),
            ]))
        );
        assert_eq!(
            js,
            Some(Value::List(vec![
                Value::String("ab".into()),
                Value::Int(0),
                Value::Int(1),
            ]))
        );
        // Structurally different, but cross-target equivalent
        assert!(matches_cross_target(&erl.unwrap(), &js.unwrap()));
    }

    // ----- Empty and edge cases -----

    #[test]
    fn parse_empty_line() {
        assert_eq!(parse_erlang(""), None);
        assert_eq!(parse_javascript(""), None);
        assert_eq!(parse_erlang("  "), None);
    }

    #[test]
    fn parse_unknown_format() {
        assert_eq!(parse_erlang("some random text"), None);
        assert_eq!(parse_javascript("some random text"), None);
    }

    // ----- split_items tests -----

    #[test]
    fn split_items_empty() {
        assert_eq!(split_erlang_items(""), Vec::<String>::new());
    }

    #[test]
    fn split_items_simple() {
        assert_eq!(split_erlang_items("1, 2, 3"), vec!["1", "2", "3"]);
    }

    #[test]
    fn split_items_nested() {
        assert_eq!(split_erlang_items("#(1, 2), 3"), vec!["#(1, 2)", "3"]);
    }

    #[test]
    fn split_items_string() {
        assert_eq!(split_erlang_items("\"a, b\", 3"), vec!["\"a, b\"", "3"]);
    }
}

#[cfg(test)]
mod real_output_tests {
    use super::*;

    const SOURCE: &str = "pub fn main() {\n  echo 42\n  echo \"hello\"\n  echo 1.0\n  echo <<1, 2, 3>>\n  echo [1, 2]\n  echo #(1, \"two\")\n  echo Ok(1)\n  echo Red\n}\n";

    const ERLANG_RAW: &str = "  Resolving versions\nDownloading packages\n Downloaded 1 package in 0.01s\n      Added gleam_stdlib v1.0.5\n  Compiling gleam_stdlib\n  Compiling main\nwarning: Unused result value\n  \u{250c}\u{2500} /private/var/folders/.../src/main.gleam:8:3\n  \u{2502}\n8 \u{2502}   echo Ok(1)\n  \u{2502}   ^^^^^^^^^^ The Result value created here is unused\n\nHint: If you are sure you don't need it you can assign it to `_`.\n\n   Compiled in 0.37s\n    Running main.main\n\u{1b}[90msrc/main.gleam:2\u{1b}[39m\n42\n\u{1b}[90msrc/main.gleam:3\u{1b}[39m\n\"hello\"\n\u{1b}[90msrc/main.gleam:4\u{1b}[39m\n1.0\n\u{1b}[90msrc/main.gleam:5\u{1b}[39m\n\"\\u{0001}\\u{0002}\\u{0003}\"\n\u{1b}[90msrc/main.gleam:6\u{1b}[39m\n[1, 2]\n\u{1b}[90msrc/main.gleam:7\u{1b}[39m\n#(1, \"two\")\n\u{1b}[90msrc/main.gleam:8\u{1b}[39m\nOk(1)\n\u{1b}[90msrc/main.gleam:9\u{1b}[39m\nRed";

    const JAVASCRIPT_RAW: &str = "  Compiling gleam_stdlib\n  Compiling main\nwarning: Unused result value\n  \u{250c}\u{2500} /private/var/folders/.../src/main.gleam:8:3\n  \u{2502}\n8 \u{2502}   echo Ok(1)\n  \u{2502}   ^^^^^^^^^^ The Result value created here is unused\n\nHint: If you are sure you don't need it you can assign it to `_`.\n\n   Compiled in 0.04s\n    Running main.main\n\u{1b}[90msrc/main.gleam:2\u{1b}[39m\n42\n\u{1b}[90msrc/main.gleam:3\u{1b}[39m\n\"hello\"\n\u{1b}[90msrc/main.gleam:4\u{1b}[39m\n1\n\u{1b}[90msrc/main.gleam:5\u{1b}[39m\n<<1, 2, 3>>\n\u{1b}[90msrc/main.gleam:6\u{1b}[39m\n[1, 2]\n\u{1b}[90msrc/main.gleam:7\u{1b}[39m\n#(1, \"two\")\n\u{1b}[90msrc/main.gleam:8\u{1b}[39m\nOk(1)\n\u{1b}[90msrc/main.gleam:9\u{1b}[39m\nRed";

    #[test]
    fn strip_build_noise_real_erlang() {
        let stripped = strip_build_noise(ERLANG_RAW);
        assert!(!stripped.contains("Resolving versions"));
        assert!(!stripped.contains("Compiling"));
        assert!(!stripped.contains("warning:"));
        assert!(!stripped.contains("Hint:"));
        assert!(!stripped.contains("Compiled in"));
        assert!(stripped.contains("42"));
        assert!(stripped.contains("\"hello\""));
        assert!(stripped.contains("Red"));
    }

    #[test]
    fn strip_build_noise_real_javascript() {
        let stripped = strip_build_noise(JAVASCRIPT_RAW);
        assert!(!stripped.contains("Compiling"));
        assert!(!stripped.contains("warning:"));
        assert!(!stripped.contains("Hint:"));
        assert!(stripped.contains("42"));
        assert!(stripped.contains("\"hello\""));
        assert!(stripped.contains("Red"));
    }

    #[test]
    fn parse_real_erlang_output() {
        let lines = echo_line_numbers(SOURCE);
        let values = parse_output_with_echo_lines(ERLANG_RAW, Target::Erlang, &lines);
        assert_eq!(
            values,
            vec![
                Value::Int(42),
                Value::String("hello".into()),
                Value::Float(1.0),
                Value::String("\u{1}\u{2}\u{3}".into()),
                Value::List(vec![Value::Int(1), Value::Int(2)]),
                Value::Tuple(vec![Value::Int(1), Value::String("two".into())]),
                Value::Constructor("Ok".into(), vec![Value::Int(1)]),
                Value::Constructor("Red".into(), vec![]),
            ]
        );
    }

    #[test]
    fn parse_real_javascript_output() {
        let lines = echo_line_numbers(SOURCE);
        let values = parse_output_with_echo_lines(JAVASCRIPT_RAW, Target::JavaScript, &lines);
        assert_eq!(
            values,
            vec![
                Value::Int(42),
                Value::String("hello".into()),
                Value::Int(1),
                Value::BitArray(vec![1, 2, 3]),
                Value::List(vec![Value::Int(1), Value::Int(2)]),
                Value::Tuple(vec![Value::Int(1), Value::String("two".into())]),
                Value::Constructor("Ok".into(), vec![Value::Int(1)]),
                Value::Constructor("Red".into(), vec![]),
            ]
        );
    }

    #[test]
    fn cross_target_matches_except_known_divergences() {
        let lines = echo_line_numbers(SOURCE);
        let erl = parse_output_with_echo_lines(ERLANG_RAW, Target::Erlang, &lines);
        let js = parse_output_with_echo_lines(JAVASCRIPT_RAW, Target::JavaScript, &lines);
        assert!(
            outputs_match_cross_target(&erl, &js),
            "erl: {:?}\njs: {:?}",
            erl,
            js
        );
    }

    // A warning with a multi-paragraph hint text (separated by blank lines)
    // must not leak into the parsed value stream. Real example captured from
    // `gleam run` output for a "Redundant tuple" warning.
    const ERLANG_MULTIPARAGRAPH_WARNING_RAW: &str = "  Compiling main\nwarning: Redundant tuple\n   \u{250c}\u{2500}/tmp/main.gleam:22:20\n   \u{2502}\n22 \u{2502}     k_seed -> case #(True, False) {\n   \u{2502}                    ^^^^^^^^^^^^^^ You can remove this tuple wrapper\n\nInstead of building a tuple and matching on it, you can match on its\ncontents directly.\nA case expression can take multiple subjects separated by commas like this:\n\n    case one_subject, another_subject {\n      _, _ -> todo\n    }\n\nSee: https://tour.gleam.run/flow-control/multiple-subjects/\n\n   Compiled in 0.25s\n    Running main.main\n\u{1b}[90msrc/main.gleam:21\u{1b}[39m\n2\n\u{1b}[90msrc/main.gleam:34\u{1b}[39m\n42\n\u{1b}[90msrc/main.gleam:47\u{1b}[39m\nTrue";

    #[test]
    fn strip_build_noise_multiparagraph_warning() {
        let stripped = strip_build_noise(ERLANG_MULTIPARAGRAPH_WARNING_RAW);
        assert!(
            !stripped.contains("Instead of building"),
            "warning hint text leaked: {stripped}"
        );
        assert!(
            !stripped.contains("A case expression can take"),
            "warning hint text leaked: {stripped}"
        );
        assert!(
            !stripped.contains("See: https://tour.gleam.run"),
            "warning hint text leaked: {stripped}"
        );
        assert!(stripped.contains("2"));
        assert!(stripped.contains("42"));
        assert!(stripped.contains("True"));
    }

    #[test]
    fn parse_multiparagraph_warning() {
        let values = parse_output_with_echo_lines(
            ERLANG_MULTIPARAGRAPH_WARNING_RAW,
            Target::Erlang,
            &echo_line_numbers(ERLANG_MULTIPARAGRAPH_SOURCE),
        );
        assert_eq!(
            values,
            vec![Value::Int(2), Value::Int(42), Value::Bool(true)]
        );
    }

    // Source corresponding to ERLANG_MULTIPARAGRAPH_WARNING_RAW. Echoes are
    // on lines 21, 34, 47.
    const ERLANG_MULTIPARAGRAPH_SOURCE: &str = r#"pub const k_golden: String = "constructor"
pub const k_limit: Float = 2.0
pub const k_seed: Int = 42

fn yield(constructor: Float, prototype: Int, default: Float) -> List(Int) {
[7]
}

fn f1(v0: Int) -> Int {
case "a" {
    item | "constructor" <> item -> {
      let item = 2.0
      4
    }
    constructor -> v0
  }
}

pub fn main() {
  let k_limit = True
  echo case 100 + 3 {
    k_seed -> case #(True, False) {
      #(False as whole, True) -> 10
      #(False, True) -> 1 + k_seed
      #(False, _) -> k_seed
      v1 -> 2
    }
    _ -> {
      let x = k_golden <> k_golden
      f1(k_seed)
    }
    3 -> 7
  }
  echo {
    let value = [10, 1]
    let constructor = value
    case "constructor" <> k_golden {
      "a" <> inner | "abc" <> inner -> 4
      inner -> k_seed
      a | "b" <> a -> {
        let a = k_seed
        let acc = k_golden
        k_seed
      }
    }
  }
  echo True
}
"#;

    #[test]
    fn echo_line_numbers_finds_all_echos() {
        let lines = echo_line_numbers(ERLANG_MULTIPARAGRAPH_SOURCE);
        assert!(lines.contains(&21), "missing echo on line 21: {lines:?}");
        assert!(lines.contains(&34), "missing echo on line 34: {lines:?}");
        assert!(lines.contains(&47), "missing echo on line 47: {lines:?}");
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn parse_with_echo_lines_ignores_warning_text() {
        // The raw output contains a "Redundant tuple" warning whose hint
        // text leaks capital-letter-prefixed lines. With source-aware
        // parsing, those lines are ignored because they don't follow an
        // echo marker.
        let lines = echo_line_numbers(ERLANG_MULTIPARAGRAPH_SOURCE);
        let values =
            parse_output_with_echo_lines(ERLANG_MULTIPARAGRAPH_WARNING_RAW, Target::Erlang, &lines);
        assert_eq!(
            values,
            vec![Value::Int(2), Value::Int(42), Value::Bool(true)],
            "warning text leaked: {values:?}"
        );
    }

    #[test]
    fn parse_with_echo_lines_handles_javascript() {
        // The JS equivalent of ERLANG_MULTIPARAGRAPH_WARNING_RAW (same
        // program, JS target).
        let raw = "  Compiling main\nwarning: Redundant tuple\n   \u{250c}\u{2500}/tmp/main.gleam:8:20\n   \u{2502}\n8 \u{2502}     k_seed -> case #(True, False) {\n   \u{2502}                    ^^^^^^^^^^^^^^ You can remove this tuple wrapper\n\nInstead of building a tuple and matching on it, you can match on its\ncontents directly.\nA case expression can take multiple subjects separated by commas like this:\n\n    case one_subject, another_subject {\n      _, _ -> todo\n    }\n\nSee: https://tour.gleam.run/flow-control/multiple-subjects/\n\n   Compiled in 0.04s\n    Running main.main\n\u{1b}[90msrc/main.gleam:21\u{1b}[39m\n2\n\u{1b}[90msrc/main.gleam:34\u{1b}[39m\n42\n\u{1b}[90msrc/main.gleam:47\u{1b}[39m\nTrue";
        let lines = echo_line_numbers(ERLANG_MULTIPARAGRAPH_SOURCE);
        let values = parse_output_with_echo_lines(raw, Target::JavaScript, &lines);
        assert_eq!(
            values,
            vec![Value::Int(2), Value::Int(42), Value::Bool(true)],
            "JS values wrong: {values:?}"
        );
    }

    #[test]
    fn parse_with_echo_lines_drops_out_of_range_markers() {
        // If a source location line in the output doesn't correspond to
        // an echo (e.g. it's a warning location), the marker is ignored
        // and the value beneath it is not picked up.
        let raw = "\x1b[90msrc/main.gleam:5\x1b[39m\nwarning text here\n\x1b[90msrc/main.gleam:11\x1b[39m\n42\n";
        let mut lines = std::collections::HashSet::new();
        lines.insert(11);
        let values = parse_output_with_echo_lines(raw, Target::Erlang, &lines);
        assert_eq!(values, vec![Value::Int(42)]);
    }
}

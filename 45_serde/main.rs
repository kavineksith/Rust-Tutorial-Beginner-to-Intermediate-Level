// ============================================================================
//  CHAPTER 47 — RUST SERDE & SERIALIZATION
//  Serde is Rust's serialization framework — the most popular crate in the
//  ecosystem. This file shows a SELF-CONTAINED manual JSON serializer
//  (compiles with plain `rustc`) plus extensive comments showing the
//  idiomatic serde setup for real projects.
// ============================================================================

use std::collections::BTreeMap;
use std::fmt::Write as _;

// ============================================================================
//  PART A — A minimal manual JSON value type (no external crates)
// ============================================================================
//  This demonstrates the same CONCEPTS serde uses, with no dependencies.
//  In real code you'd use serde_json — see Part B.

#[derive(Debug, Clone, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

impl Json {
    /// Serialize this Json value to a String.
    fn to_string_pretty(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, 0);
        out
    }

    fn write(&self, out: &mut String, indent: usize) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(out, "{}", *n as i64).unwrap();
                } else {
                    write!(out, "{}", n).unwrap();
                }
            }
            Json::String(s) => {
                out.push('"');
                for c in s.chars() {
                    match c {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        '\t' => out.push_str("\\t"),
                        c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32).unwrap(),
                        c => out.push(c),
                    }
                }
                out.push('"');
            }
            Json::Array(arr) => {
                if arr.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push_str("[\n");
                for (i, item) in arr.iter().enumerate() {
                    out.push_str(&"  ".repeat(indent + 1));
                    item.write(out, indent + 1);
                    if i + 1 < arr.len() { out.push(','); }
                    out.push('\n');
                }
                out.push_str(&"  ".repeat(indent));
                out.push(']');
            }
            Json::Object(map) => {
                if map.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push_str("{\n");
                let total = map.len();
                for (i, (k, v)) in map.iter().enumerate() {
                    out.push_str(&"  ".repeat(indent + 1));
                    Json::String(k.clone()).write(out, indent + 1);
                    out.push_str(": ");
                    v.write(out, indent + 1);
                    if i + 1 < total { out.push(','); }
                    out.push('\n');
                }
                out.push_str(&"  ".repeat(indent));
                out.push('}');
            }
        }
    }
}

// A tiny JSON parser (just enough for demo purposes):
impl Json {
    fn parse(input: &str) -> Result<Json, String> {
        let mut chars = input.chars().peekable();
        let v = Self::parse_value(&mut chars)?;
        Self::skip_ws(&mut chars);
        if chars.peek().is_some() {
            return Err("trailing characters".to_string());
        }
        Ok(v)
    }

    fn skip_ws(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() { chars.next(); } else { break; }
        }
    }

    fn parse_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Json, String> {
        Self::skip_ws(chars);
        match chars.peek() {
            Some('{') => Self::parse_object(chars),
            Some('[') => Self::parse_array(chars),
            Some('"') => Ok(Json::String(Self::parse_string(chars)?)),
            Some('t') | Some('f') => Self::parse_bool(chars),
            Some('n') => Self::parse_null(chars),
            Some(c) if c.is_ascii_digit() || *c == '-' => Self::parse_number(chars),
            Some(c) => Err(format!("unexpected char: {}", c)),
            None => Err("unexpected end of input".to_string()),
        }
    }

    fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
        chars.next(); // consume opening quote
        let mut s = String::new();
        while let Some(c) = chars.next() {
            match c {
                '"' => return Ok(s),
                '\\' => match chars.next() {
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some(c) => return Err(format!("bad escape: \\{}", c)),
                    None => return Err("unterminated escape".to_string()),
                },
                c => s.push(c),
            }
        }
        Err("unterminated string".to_string())
    }

    fn parse_bool(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Json, String> {
        let mut word = String::new();
        for _ in 0..5 {
            if let Some(&c) = chars.peek() {
                if c.is_alphabetic() { word.push(c); chars.next(); }
                else { break; }
            }
        }
        match word.as_str() {
            "true" => Ok(Json::Bool(true)),
            "false" => Ok(Json::Bool(false)),
            w => Err(format!("invalid literal: {}", w)),
        }
    }

    fn parse_null(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Json, String> {
        let mut word = String::new();
        for _ in 0..4 {
            if let Some(&c) = chars.peek() {
                if c.is_alphabetic() { word.push(c); chars.next(); }
                else { break; }
            }
        }
        if word == "null" { Ok(Json::Null) } else { Err(format!("invalid: {}", word)) }
    }

    fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Json, String> {
        let mut s = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() || c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E' {
                s.push(c);
                chars.next();
            } else { break; }
        }
        s.parse::<f64>().map(Json::Number).map_err(|e| e.to_string())
    }

    fn parse_array(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Json, String> {
        chars.next(); // [
        let mut arr = Vec::new();
        Self::skip_ws(chars);
        if chars.peek() == Some(&']') { chars.next(); return Ok(Json::Array(arr)); }
        loop {
            arr.push(Self::parse_value(chars)?);
            Self::skip_ws(chars);
            match chars.next() {
                Some(',') => continue,
                Some(']') => return Ok(Json::Array(arr)),
                Some(c) => return Err(format!("expected , or ], got {}", c)),
                None => return Err("unterminated array".to_string()),
            }
        }
    }

    fn parse_object(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Json, String> {
        chars.next(); // {
        let mut map = BTreeMap::new();
        Self::skip_ws(chars);
        if chars.peek() == Some(&'}') { chars.next(); return Ok(Json::Object(map)); }
        loop {
            Self::skip_ws(chars);
            let key = Self::parse_string(chars)?;
            Self::skip_ws(chars);
            match chars.next() {
                Some(':') => {}
                Some(c) => return Err(format!("expected :, got {}", c)),
                None => return Err("unterminated object".to_string()),
            }
            let val = Self::parse_value(chars)?;
            map.insert(key, val);
            Self::skip_ws(chars);
            match chars.next() {
                Some(',') => continue,
                Some('}') => return Ok(Json::Object(map)),
                Some(c) => return Err(format!("expected , or }}, got {}", c)),
                None => return Err("unterminated object".to_string()),
            }
        }
    }
}

fn main() {
    println!("=== Serde & Serialization (manual JSON demo) ===\n");

    // ---- Build a JSON value by hand ----
    let mut user = BTreeMap::new();
    user.insert("name".to_string(), Json::String("Ada Lovelace".to_string()));
    user.insert("age".to_string(), Json::Number(36.0));
    user.insert("active".to_string(), Json::Bool(true));
    user.insert("tags".to_string(), Json::Array(vec![
        Json::String("math".to_string()),
        Json::String("computing".to_string()),
    ]));
    user.insert("address".to_string(), Json::Null);

    let value = Json::Object(user);
    let serialized = value.to_string_pretty();
    println!("serialized:\n{}\n", serialized);

    // ---- Parse it back ----
    let parsed = Json::parse(&serialized).expect("parse failed");
    println!("parsed back: {:?}\n", parsed);
    assert_eq!(value, parsed);
    println!("round-trip OK!");

    // ========================================================================
    //  PART B — REAL SERDE SETUP (requires cargo + dependencies)
    // ========================================================================
    //
    //  Cargo.toml:
    //    [dependencies]
    //    serde = { version = "1", features = ["derive"] }
    //    serde_json = "1"
    //    # Optional, for other formats:
    //    serde_yaml = "0.9"
    //    toml = "0.8"
    //    bincode = "1"
    //
    //  src/main.rs:
    //
    //    use serde::{Serialize, Deserialize};
    //
    //    #[derive(Serialize, Deserialize, Debug)]
    //    struct User {
    //        name: String,
    //        age: u32,
    //        active: bool,
    //        tags: Vec<String>,
    //        address: Option<String>,
    //    }
    //
    //    fn main() -> Result<(), serde_json::Error> {
    //        let user = User {
    //            name: "Ada Lovelace".to_string(),
    //            age: 36,
    //            active: true,
    //            tags: vec!["math".to_string(), "computing".to_string()],
    //            address: None,
    //        };
    //
    //        // Serialize to JSON string:
    //        let json = serde_json::to_string_pretty(&user)?;
    //        println!("{}", json);
    //
    //        // Deserialize from JSON string:
    //        let parsed: User = serde_json::from_str(&json)?;
    //        println!("{:?}", parsed);
    //
    //        Ok(())
    //    }
    //
    //  ========================================================================
    //  SERDE ATTRIBUTES (customizing serialization)
    //  ========================================================================
    //
    //    #[derive(Serialize, Deserialize)]
    //    struct User {
    //        // Rename a field:
    //        #[serde(rename = "userName")]
    //        name: String,
    //
    //        // Rename all fields (camelCase, snake_case, PascalCase, ...):
    //        #[serde(rename_all = "camelCase")]
    //        // ... fields ...
    //
    //        // Skip a field:
    //        #[serde(skip)]
    //        cached_password: String,
    //
    //        // Skip if None (omit from output when None):
    //        #[serde(skip_serializing_if = "Option::is_none")]
    //        address: Option<String>,
    //
    //        // Default value if missing in input:
    //        #[serde(default)]
    //        tags: Vec<String>,
    //
    //        // Default with a custom function:
    //        #[serde(default = "default_count")]
    //        count: u32,
    //
    //        // Use a different serialization function:
    //        #[serde(serialize_with = "serialize_timestamp")]
    //        created_at: i64,
    //
    //        // Flatten a nested struct into the parent:
    //        #[serde(flatten)]
    //        metadata: Metadata,
    //
    //        // Tagged enum representation:
    //        #[serde(tag = "type")]
    //        // enum Shape { Circle { radius: f64 }, Square { side: f64 } }
    //    }
    //
    //    fn default_count() -> u32 { 0 }
    //
    //  ========================================================================
    //  ENUM REPRESENTATIONS
    //  ========================================================================
    //
    //    #[derive(Serialize, Deserialize)]
    //    #[serde(tag = "type", rename_all = "lowercase")]
    //    enum Message {
    //        Quit,
    //        Move { x: i32, y: i32 },
    //        Write(String),
    //    }
    //
    //    // Serializes as:
    //    // {"type": "quit"}
    //    // {"type": "move", "x": 1, "y": 2}
    //    // {"type": "write", "Write": "hi"}    // (the Write variant's value uses the variant name)
    //
    //  ========================================================================
    //  SERDE WITH OTHER FORMATS
    //  ========================================================================
    //
    //    // YAML:
    //    let yaml = serde_yaml::to_string(&user)?;
    //    let user: User = serde_yaml::from_str(&yaml)?;
    //
    //    // TOML:
    //    let toml = toml::to_string(&user)?;
    //    let user: User = toml::from_str(&toml)?;
    //
    //    // Bincode (compact binary):
    //    let bytes = bincode::serialize(&user)?;
    //    let user: User = bincode::deserialize(&bytes)?;
    //
    //  ========================================================================
    //  CUSTOM SERIALIZE / DESERIALIZE
    //  ========================================================================
    //
    //  When derive isn't enough, implement Serialize and Deserialize manually:
    //
    //    use serde::{Serializer, Deserializer, Deserialize};
    //
    //    impl Serialize for MyType {
    //        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    //            serializer.serialize_str(&format!("{:?}", self))
    //        }
    //    }
    //
    //    impl<'de> Deserialize<'de> for MyType {
    //        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    //            let s = String::deserialize(d)?;
    //            s.parse().map_err(serde::de::Error::custom)
    //        }
    //    }
    //  ========================================================================
}

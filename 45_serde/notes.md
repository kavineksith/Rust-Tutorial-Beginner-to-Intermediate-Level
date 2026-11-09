# Serde & Serialization

**Serde** (SERialize / DEserialize) is Rust's most popular crate — it
provides a single framework for converting between Rust types and data
formats (JSON, YAML, TOML, bincode, MessagePack, CSV, ...).

## Quick Start

`Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    active: bool,
    tags: Vec<String>,
    address: Option<String>,
}

fn main() -> Result<(), serde_json::Error> {
    let user = User {
        name: "Ada Lovelace".to_string(),
        age: 36,
        active: true,
        tags: vec!["math".to_string(), "computing".to_string()],
        address: None,
    };

    // Serialize to JSON:
    let json = serde_json::to_string_pretty(&user)?;
    println!("{}", json);

    // Deserialize from JSON:
    let parsed: User = serde_json::from_str(&json)?;
    println!("{:?}", parsed);

    Ok(())
}
```

Output:

```json
{
  "name": "Ada Lovelace",
  "age": 36,
  "active": true,
  "tags": [
    "math",
    "computing"
  ],
  "address": null
}
```

## How It Works

* `#[derive(Serialize, Deserialize)]` generates code that calls into
  serde's `Serializer` / `Deserializer` traits.
* Each format (JSON, YAML, etc.) implements those traits.
* One derive works for **every** format — just call the format's function.

## Common Attributes

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]   // applies to all fields
struct User {
    #[serde(rename = "userId")]      // rename a single field
    id: u64,

    #[serde(skip)]                   // skip serialization entirely
    cached_password: String,

    #[serde(skip_serializing_if = "Option::is_none")]  // omit if None
    address: Option<String>,

    #[serde(default)]                // use Default::default() if missing
    tags: Vec<String>,

    #[serde(default = "default_count")]   // use a function if missing
    count: u32,

    #[serde(flatten)]                // pull nested fields up to parent
    metadata: Metadata,
}

fn default_count() -> u32 { 0 }
```

### Renaming

| Attribute | Effect |
|-----------|--------|
| `#[serde(rename = "name")]` | rename one field/variant |
| `#[serde(rename_all = "camelCase")]` | rename all fields/variants |
| `rename_all` values | `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, `SCREAMING-KEBAB-CASE` |

### Skipping

| Attribute | Effect |
|-----------|--------|
| `#[serde(skip)]` | skip when serializing AND deserializing (uses `Default`) |
| `#[serde(skip_serializing)]` | skip only when serializing |
| `#[serde(skip_deserializing)]` | skip only when deserializing |
| `#[serde(skip_serializing_if = "Option::is_none")]` | conditional skip |

### Defaults

```rust
#[serde(default)]                  // uses Default::default()
tags: Vec<String>,

#[serde(default = "default_count")]  // uses the named function
count: u32,
```

### Flattening

```rust
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    #[serde(flatten)]
    metadata: Metadata,   // fields of Metadata appear at the top level
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    created_at: i64,
    updated_at: i64,
}
```

Result:

```json
{
  "name": "Ada",
  "created_at": 1234567890,
  "updated_at": 1234567890
}
```

## Enum Representations

By default, serde serializes enums as externally tagged:

```rust
#[derive(Serialize, Deserialize)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

```json
{"Move": {"x": 1, "y": 2}}
{"Write": "hello"}
{"Quit": null}
```

### Internally Tagged

```rust
#[serde(tag = "type")]
```

```json
{"type": "Move", "x": 1, "y": 2}
{"type": "Write", "Write": "hello"}   // hmm, the Write field name is awkward
```

### Adjacently Tagged

```rust
#[serde(tag = "type", content = "data")]
```

```json
{"type": "Move", "data": {"x": 1, "y": 2}}
{"type": "Write", "data": "hello"}
```

### Untagged

```rust
#[serde(untagged)]
```

```json
{"x": 1, "y": 2}      // inferred to be Move
"hello"               // inferred to be Write
null                  // inferred to be Quit
```

Useful for matching JSON shapes; can be slower and produce worse errors.

### Rename Variants

```rust
#[serde(rename_all = "lowercase")]
enum Message {
    Quit,         // "quit"
    Move,         // "move"
    Write,        // "write"
}
```

## Using Different Formats

Just swap the format crate — the struct stays the same:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
toml = "0.8"
bincode = "1"
```

```rust
// JSON
let json = serde_json::to_string(&user)?;
let user: User = serde_json::from_str(&json)?;

// YAML
let yaml = serde_yaml::to_string(&user)?;
let user: User = serde_yaml::from_str(&yaml)?;

// TOML
let toml = toml::to_string(&user)?;
let user: User = toml::from_str(&toml)?;

// Bincode (compact binary)
let bytes = bincode::serialize(&user)?;
let user: User = bincode::deserialize(&bytes)?;
```

## Working with `serde_json::Value`

When you don't know the shape at compile time, use `Value`:

```rust
use serde_json::json;

let v = json!({
    "name": "Ada",
    "age": 36,
    "tags": ["math", "computing"],
});

println!("{}", v["name"]);           // "Ada"
println!("{}", v["tags"][0]);        // "math"

// Match on the value:
match &v["age"] {
    serde_json::Value::Number(n) => println!("age = {}", n),
    _ => println!("age is not a number"),
}
```

### Parsing Arbitrary JSON

```rust
let json = r#"{"a": 1, "b": [2, 3]}"#;
let v: serde_json::Value = serde_json::from_str(json)?;
println!("{}", v["b"][1]);   // 3
```

## Streaming JSON

For large JSON files, use the streaming API:

```rust
use serde_json::{Deserializer, from_str};

let json = r#"{"a":1}{"b":2}{"c":3}"#;

let stream = Deserializer::from_str(json).into_iter::<serde_json::Value>();
for value in stream {
    let value = value?;
    println!("{:?}", value);
}
```

This reads one JSON value at a time — memory-efficient for big files.

## Custom Serialize / Deserialize

When derive isn't enough:

```rust
use serde::{Serializer, Deserializer, Deserialize};

impl serde::Serialize for MyType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

impl<'de> serde::Deserialize<'de> for MyType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
```

Most of the time, derive + attributes is enough.

## Serde with Generics

```rust
#[derive(Serialize, Deserialize)]
struct Wrapper<T> {
    value: T,
}

let w = Wrapper { value: 42 };
let json = serde_json::to_string(&w)?;
```

The generic `T` must itself implement `Serialize` / `Deserialize`.

## Serde Trait Bounds

`#[derive(Serialize)]` adds `T: Serialize` bounds to generic types. To
opt out, use bounds in `impl` blocks instead:

```rust
#[derive(Serialize)]
struct Wrapper<T> {
    value: T,
}

// This impl block only applies when T: Clone:
impl<T: Clone> Wrapper<T> {
    fn cloned_value(&self) -> T { self.value.clone() }
}
```

For complex cases, use `serde_with` or write manual impls.

## Common Pitfalls

### Forgetting `#[derive(Serialize, Deserialize)]`

```rust
struct User { name: String }   // ERROR: trait bound not satisfied
serde_json::to_string(&user)?;
```

Add the derive.

### Missing Fields in JSON

```rust
#[derive(Deserialize)]
struct User { name: String, age: u32 }

// Input: {"name": "Ada"}
// ERROR: missing field `age`
```

Add `#[serde(default)]` to make `age` optional, or change to `Option<u32>`.

### Wrong Type

```rust
// Input: {"age": "36"}  (string instead of number)
// ERROR: invalid type: string "36", expected u32
```

Make sure your data matches your types, or implement a custom deserializer.

### Lifetimes in Deserialized Structs

```rust
// Hard — JSON borrows from the input string:
#[derive(Deserialize)]
struct Borrowed<'a> { name: &'a str }

let s: Borrowed = serde_json::from_str("{\"name\":\"hi\"}")?;
// s.name borrows from the input string — input must outlive s
```

Use `String` instead of `&str` for owned data.

## Useful Crates

| Crate | Purpose |
|-------|---------|
| `serde` | the framework |
| `serde_json` | JSON support |
| `serde_yaml` | YAML |
| `toml` | TOML |
| `bincode` | compact binary |
| `serde_cbor` | CBOR |
| `rmp-serde` | MessagePack |
| `serde_with` | extra attributes (e.g. `serde_with::skip_serializing_none`) |
| `serde_repr` | serialize C-like enums as their integer value |

## Cheat Sheet

```rust
// Add to Cargo.toml:
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

// Serialize:
let json = serde_json::to_string(&user)?;
let pretty = serde_json::to_string_pretty(&user)?;

// Deserialize:
let user: User = serde_json::from_str(&json)?;
let user: User = serde_json::from_slice(&bytes)?;
let user: User = serde_json::from_reader(file)?;

// Dynamic JSON:
let v: serde_json::Value = serde_json::from_str(&json)?;
let v = json!({ "key": "value" });
```

**Next:** [Logging & Tracing](../46_logging) — `log`, `env_logger`, `tracing`.

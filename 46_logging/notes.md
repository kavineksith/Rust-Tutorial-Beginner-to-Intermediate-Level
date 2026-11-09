# Logging & Tracing

Rust's logging story has two layers:

1. The **`log` crate** — a logging facade (interface). Libraries use it to
   emit log messages without choosing a backend.
2. A **backend** like `env_logger` (simple text) or `tracing` (structured,
   async-aware).

## Quick Start: `log` + `env_logger`

```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

```rust
use log::{info, warn, error, debug, trace};

fn main() {
    env_logger::init();

    info!("starting application");
    debug!("debug info: x = {}", 42);
    warn!("something suspicious");
    error!("something failed");
}
```

Run with `RUST_LOG` env var to control verbosity:

```bash
RUST_LOG=info cargo run
RUST_LOG=debug cargo run
RUST_LOG=trace cargo run
```

## Log Levels

| Level | When to use |
|-------|-------------|
| `error!` | operation failed; user-visible problem |
| `warn!` | unexpected situation, but operation continued |
| `info!` | high-level progress (server started, request handled) |
| `debug!` | detailed info for diagnosing problems |
| `trace!` | very detailed (every iteration, every function call) |

Production typically runs at `INFO`. Debug builds at `DEBUG`. Deep
investigations at `TRACE`.

## Filtering with `RUST_LOG`

```bash
RUST_LOG=info                          # everything at info or higher
RUST_LOG=debug                         # everything at debug or higher
RUST_LOG=my_crate=debug                # only my_crate at debug
RUST_LOG=my_crate::network=trace       # only one module at trace
RUST_LOG=debug,hyper=warn              # this crate at debug, hyper at warn
RUST_LOG=off                           # silence everything
```

## Why a Facade?

Libraries use `log::info!` etc. without depending on a backend. The
application chooses the backend (env_logger, tracing, simple_logger, ...).
This means **any** library's logs work with **any** backend.

## Backends Comparison

| Backend | Use case |
|---------|----------|
| `env_logger` | standard, simple, configurable via `RUST_LOG` |
| `simple_logger` | easy setup, no env var needed |
| `flexi_logger` | log rotation, file output, customization |
| `fern` | flexible configuration with chaining |
| `tracing-subscriber` | structured, async-aware, JSON output |
| `slog` | structured, hierarchical (older alternative) |

## Structured Logging with `tracing`

For servers, async apps, or log aggregation:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tracing::{info, debug, span, Level};

fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "request", request_id = "abc123");
    let _enter = span.enter();

    info!("handling request");
    debug!(?config, "loaded config");
}
```

Output:

```
INFO request{request_id="abc123"}: handling request
DEBUG request{request_id="abc123"}: loaded config config=Config{...}
```

### Spans

A **span** represents a unit of work. All logs inside the span are tagged
with its context. Spans can nest:

```rust
use tracing::{span, Level, info};

let app_span = span!(Level::INFO, "app");
let _app = app_span.enter();

let request_span = span!(Level::INFO, "request", id = %request_id);
let _req = request_span.enter();

info!("processing");   // inside both spans
```

### Span Timing

`tracing` automatically records span entry/exit times. With
`tracing-subscriber`'s `fmt` layer, you can see how long each span took:

```rust
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
    .init();
```

Output:

```
INFO request{request_id="abc123"}: processing
INFO request{request_id="abc123"}: close, time: 1.23ms
```

### JSON Output

For log aggregation (Elasticsearch, Datadog, ...):

```rust
tracing_subscriber::fmt()
    .json()
    .init();
```

## `tracing` in Async Code

`tracing` is built for async — spans survive across `.await` points:

```rust
use tracing::instrument;

#[instrument]
async fn handle_request(req: Request) -> Response {
    // automatically wrapped in a span named "handle_request"
    // with `req` as a field
    let user = authenticate(&req).await?;
    let data = fetch_data(&user).await?;
    Ok(build_response(data))
}
```

`#[instrument]` generates a span that captures all function arguments as
fields. Beautiful for tracing request flow through async code.

## Logging in Libraries

A library should use `log` or `tracing`, but should **not** initialize a
backend — that's the application's job.

```rust
// In your library:
use log::{info, warn};

pub fn process(input: &str) -> Result<(), Error> {
    info!("processing {} bytes", input.len());
    // ...
    Ok(())
}
```

If the application hasn't initialized a backend, log calls become no-ops
(zero cost). When they DO initialize one, your library's logs appear.

## Logging vs Errors

Don't log AND return an error — pick one.

```rust
// BAD: duplicates the error info
fn fetch(url: &str) -> Result<String, Error> {
    let resp = http::get(url).map_err(|e| {
        error!("fetch failed: {}", e);
        e
    })?;
    Ok(resp.body)
}

// GOOD: return the error, let the caller decide whether to log
fn fetch(url: &str) -> Result<String, Error> {
    let resp = http::get(url)?;
    Ok(resp.body)
}

// Caller:
match fetch(url) {
    Ok(body) => info!("fetched {} bytes", body.len()),
    Err(e)   => error!("fetch failed: {}", e),
}
```

Libraries especially should not log errors — return them, let the
application log/alert/ignore as it sees fit.

## Best Practices

| Do | Don't |
|----|-------|
| Log high-level progress at `info!` | Log every loop iteration |
| Log failures at `error!` | Log + return the same error |
| Use structured fields (`info!(?user, ...)`) | Build strings with `format!` |
| Avoid logging in hot paths | Log inside tight loops |
| Don't log secrets (passwords, tokens) | Log raw HTTP headers |
| Initialize logging once at startup | Call `env_logger::init()` in a library |

## Structured Fields

Use the `?` (Debug) and `%` (Display) sigils:

```rust
let user = User { name: "Ada", id: 42 };

info!(?user, "user logged in");          // uses Debug
info!(%user.name, "greeting user");      // uses Display
info!(user.id, "user id");               // field name = expression
```

Output:

```
INFO user logged in user=User { name: "Ada", id: 42 }
INFO greeting user user.name=Ada
INFO user id user.id=42
```

## Custom Subscribers

`tracing-subscriber` is composable:

```rust
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

let fmt_layer = fmt::layer().with_target(false);
let filter = EnvFilter::from_default_env();

tracing_subscriber::registry()
    .with(filter)
    .with(fmt_layer)
    .init();
```

Add multiple layers (e.g. fmt + JSON + OpenTelemetry) for different sinks.

## Logging Crates Cheat Sheet

| Crate | Purpose |
|-------|---------|
| `log` | logging facade (interface) |
| `env_logger` | simple backend, `RUST_LOG` env var |
| `simple_logger` | easy setup, no env var |
| `flexi_logger` | log rotation, file output |
| `fern` | flexible config |
| `tracing` | structured, async-aware |
| `tracing-subscriber` | subscribers / formatters for tracing |
| `tracing-appender` | file appender for tracing |
| `slog` | structured, hierarchical (older) |
| `log4rs` | log4j-style config |

## Quick Reference

```rust
// Initialize (once, in main):
env_logger::init();

// Log at various levels:
error!("critical failure: {}", e);
warn!("suspicious: {}", x);
info!("server started on port {}", port);
debug!("state: {:?}", state);
trace!("entered function foo");

// With tracing:
use tracing::instrument;

#[instrument]
fn process(user_id: u64) -> Result<()> {
    // automatically wrapped in a span
    // ...
}
```

Run with `RUST_LOG=info` (or `debug`, `trace`) to see output.

**Next:** [unsafe Rust & FFI](../47_unsafe_ffi) — raw pointers, calling C.

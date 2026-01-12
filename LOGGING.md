# Logging Configuration Guide

## 📝 Current Setup

Your application now has a **dual logging system**:

1. **Console Output** (Terminal) - With colors, for development
2. **File Output** (`./logs/server.log`) - Without colors, for production/debugging

## 🎯 How It Works

### Log Levels

The application supports multiple log levels (from most to least verbose):

```rust
tracing::trace!("Very detailed information");
tracing::debug!("Debugging information");
tracing::info!("General information");
tracing::warn!("Warning messages");
tracing::error!("Error messages");
```

### Current Configuration

- **Default Level**: `INFO` (shows info, warn, and error messages)
- **Console**: Colored output via `stdout`
- **File**: Daily rotating logs in `./logs/server.log.YYYY-MM-DD`

## 🔧 Changing Log Levels

### Option 1: Environment Variable (Recommended)

Set the `RUST_LOG` environment variable:

```bash
# Show all logs including debug
RUST_LOG=debug cargo watch -x run

# Show only warnings and errors
RUST_LOG=warn cargo watch -x run

# Show everything (very verbose)
RUST_LOG=trace cargo watch -x run

# Module-specific logging
RUST_LOG=rust_boilerplate=debug cargo watch -x run

# Multiple modules with different levels
RUST_LOG=rust_boilerplate=debug,axum=info cargo watch -x run
```

### Option 2: .env File

Create/update `.env` file:

```env
RUST_LOG=debug
```

Then run normally:

```bash
cargo watch -x run
```

## 📊 Using Logging in Your Code

### In Controllers

```rust
use tracing::{info, debug, warn, error};

pub async fn create_user(payload: CreateUserDto) -> Result<UserResponse> {
    info!("Creating new user: {}", payload.username);
    debug!("User payload: {:?}", payload);

    // ... your logic

    info!("User created successfully with id: {}", user.id);
    Ok(user)
}
```

### In Services

```rust
pub async fn process_data(data: &str) -> Result<ProcessedData> {
    debug!("Processing data: {}", data);

    match validate_data(data) {
        Ok(_) => {
            info!("Data validation passed");
        },
        Err(e) => {
            warn!("Data validation failed: {}", e);
            return Err(e);
        }
    }

    // ... processing logic
}
```

### Error Handling

```rust
match dangerous_operation().await {
    Ok(result) => {
        info!("Operation successful: {:?}", result);
        Ok(result)
    },
    Err(e) => {
        error!("Operation failed: {}", e);
        Err(e)
    }
}
```

## 📁 Log Files

### Location

All logs are stored in: `./logs/`

### File Naming

- Format: `server.log.YYYY-MM-DD`
- Example: `server.log.2026-01-12`
- **Rotation**: Daily (automatically creates new file each day)

### .gitignore

The `/logs` directory is already ignored in git, so your logs won't be committed.

## 🎨 Example Output

### Terminal (with colors):

```
2026-01-12T09:12:46.192343Z  INFO rust_boilerplate: ✅ Server successfully started at http://127.0.0.1:3000
2026-01-12T09:12:46.192429Z  INFO rust_boilerplate: 📝 Logs are being written to: ./logs/server.log
```

### File (without colors):

```
2026-01-12T09:12:46.192343Z  INFO rust_boilerplate: Server successfully started at http://127.0.0.1:3000
2026-01-12T09:12:46.192429Z  INFO rust_boilerplate: Logs are being written to: ./logs/server.log
```

## 🔍 Viewing Logs

### Follow live logs in terminal

```bash
# While server is running
cargo watch -x run

# Or watch the log file
tail -f logs/server.log.$(date +%Y-%m-%d)
```

### Search logs

```bash
# Find all errors
grep ERROR logs/server.log.*

# Find specific patterns
grep "user created" logs/server.log.*

# Last 50 lines
tail -n 50 logs/server.log.2026-01-12
```

## 🚀 Production Recommendations

For production, consider:

1. **Set appropriate log level**:

   ```env
   RUST_LOG=info  # Don't use debug/trace in production
   ```

2. **Log rotation**: Already configured (daily rotation)

3. **Log monitoring**: Use tools like:

   - `logrotate` for cleanup
   - ELK Stack (Elasticsearch, Logstash, Kibana)
   - Grafana Loki
   - CloudWatch (AWS)

4. **Structured logging**: Consider adding JSON formatting for production:
   ```rust
   // Add to Cargo.toml
   tracing-subscriber = { version = "0.3", features = ["json"] }
   ```

## 📖 More Information

- [Tracing Docs](https://docs.rs/tracing/)
- [Tracing Subscriber Docs](https://docs.rs/tracing-subscriber/)
- [Tracing Appender Docs](https://docs.rs/tracing-appender/)

---

**Note**: To see debug logs, remember to set `RUST_LOG=debug` before running!

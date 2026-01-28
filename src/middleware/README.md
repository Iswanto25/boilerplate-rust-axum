# HTTP Request Logger Middleware

Morgan-style HTTP request logger untuk Axum dengan format yang clean dan informatif.

## Format Output

```
METHOD PATH STATUS DURATION
```

### Contoh:

```
GET /users 200 15.123ms
POST /api/login 401 5.234ms
GET /health 200 2.001ms
```

## Fitur

- ✅ **Log Level berdasarkan Status Code**:

  - `2xx` → `INFO`
  - `3xx` → `INFO`
  - `4xx` → `WARN`
  - `5xx` → `ERROR`

- ✅ **Response Time** dalam milliseconds dengan 3 desimal
- ✅ **Automatic Color** di terminal (via tracing configuration)
- ✅ **Clean output** di file logs (tanpa ANSI colors)

## Penggunaan

Middleware sudah terintegrasi di `main.rs`:

```rust
use axum::middleware as axum_middleware;

let app = routes::routes::get_routes()
    .layer(axum_middleware::from_fn(middleware::logger_middleware))
    .with_state(state);
```

## Output Examples

### Terminal (dengan warna)

```
2026-01-13T17:44:50.123Z INFO GET /users 200 15.123ms
2026-01-13T17:44:51.456Z WARN POST /api/login 401 5.234ms
2026-01-13T17:44:52.789Z ERROR GET /api/error 500 100.567ms
```

### File Log (tanpa warna)

```
2026-01-13T17:44:50.123Z INFO GET /users 200 15.123ms
2026-01-13T17:44:51.456Z WARN POST /api/login 401 5.234ms
2026-01-13T17:44:52.789Z ERROR GET /api/error 500 100.567ms
```

## Implementasi

File: `src/middleware/logger.rs`

Middleware ini:

1. Capture request method dan URI
2. Start timer sebelum request processing
3. Call next handler/middleware
4. Calculate elapsed time
5. Log dengan format Morgan-style
6. Return response

## HTTP Status Code Mapping

| Status Range | Log Level | Description       |
| ------------ | --------- | ----------------- |
| 200-299      | INFO      | Success responses |
| 300-399      | INFO      | Redirections      |
| 400-499      | WARN      | Client errors     |
| 500-599      | ERROR     | Server errors     |
| Others       | INFO      | Unknown status    |

## Customization

Jika ingin mengubah format log, edit fungsi `logger_middleware` di `src/middleware/logger.rs`.

Contoh format custom:

```rust
tracing::info!(
    "[{}] {} {} - {}ms",
    status.as_u16(),
    method,
    uri.path(),
    duration_ms
);
```

# Config Module

Module ini berisi konfigurasi aplikasi Rust yang terorganisir dengan baik.

## Struktur

- **`env.rs`** - Main configuration struct dan environment variable loading
- **`database.rs`** - Database initialization dan connection pooling
- **`logging.rs`** - Logging configuration dengan dual output (console + file)
- **`mod.rs`** - Module exports

## Penggunaan

### 1. Load Config dari Environment

```rust
use config::Config;

let config = Config::from_env()
    .expect("Failed to load configuration");
```

### 2. Initialize Logging

```rust
config::init_logging(&config.logging);
```

### 3. Initialize Database (Optional)

```rust
let db = config::init_database(&config.database)
    .await
    .expect("Failed to initialize database");

// Test connection
config::test_connection(&db)
    .await
    .expect("Database connection test failed");
```

### 4. Akses Konfigurasi

```rust
// Server address
let addr = config.server_address();

// Environment checks
if config.is_production() {
    // production-only code
}

if config.is_development() {
    // development-only code
}

// Individual configs
println!("Host: {}", config.server.host);
println!("Port: {}", config.server.port);
println!("DB URL: {}", config.database.url);
println!("JWT Secret: {}", config.jwt.secret);
```

## Environment Variables

Lihat file `.env.example` untuk daftar lengkap environment variables yang tersedia.

### Required Variables

- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key untuk JWT token

### Optional Variables (dengan defaults)

- `HOST` - Server host (default: `127.0.0.1`)
- `PORT` - Server port (default: `3000`)
- `ENVIRONMENT` - Environment mode (default: `development`)
- `DB_MAX_CONNECTIONS` - Max database connections (default: `10`)
- `DB_MIN_CONNECTIONS` - Min database connections (default: `2`)
- `DB_CONNECT_TIMEOUT` - DB connect timeout in seconds (default: `30`)
- `JWT_EXPIRATION` - JWT expiration in seconds (default: `86400` / 24 hours)
- `LOG_LEVEL` - Log level (default: `info`)
- `LOG_DIR` - Log directory (default: `logs`)
- `LOG_FILE` - Log filename (default: `server.log`)

## Best Practices

1. **Jangan commit file `.env`** - Gunakan `.env.example` untuk dokumentasi
2. **Gunakan strong JWT_SECRET di production** - Generate random string yang kuat
3. **Set ENVIRONMENT** dengan benar di production (`production`, `staging`, atau `development`)
4. **Sesuaikan database pool size** berdasarkan kebutuhan aplikasi
5. **Gunakan environment-specific log levels** (debug untuk dev, info/warn untuk production)

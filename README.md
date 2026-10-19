# rust-starter

A practical starter for Rust HTTP API services using Axum and Tokio.

## Included

- Separate development and production environment files with checked-in
  [development](.env.development.example) and [production](.env.production.example) examples.
- `GET /health` readiness endpoint and versioned `/api/v1` router.
- JSON request validation and consistent JSON error responses.
- Request IDs, structured tracing, CORS, body-size limits, and graceful shutdown.
- Unit and HTTP integration tests, Clippy/rustfmt configuration, and GitHub Actions CI.

## Start a new service

1. Rename the crate and update the package metadata in `Cargo.toml`.
2. Replace `AppState` and the example `GET /api/v1/greeting` endpoint with your domain.
3. Copy `.env.development.example` to `.env.development` and adjust local settings as needed.

```shell
cargo run
```

For production, copy `.env.production.example` to `.env.production`, adjust the settings,
and select it with `APP_ENV`:

```powershell
$env:APP_ENV = "production"
cargo run --release
```

`APP_ENV` accepts `development` (the default) or `production`, independently of the Cargo build
profile. The selected file is loaded from the working directory. Existing shell
environment variables take precedence. A missing file is allowed for deployments
that provide all settings through environment variables; malformed files fail at
startup. The old `.env` file is no longer loaded. Both local files are ignored by Git.

The server listens at `http://127.0.0.1:3000` by default.

```shell
curl http://127.0.0.1:3000/health
curl 'http://127.0.0.1:3000/api/v1/greeting?name=Ada'
```

## Checks

```shell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps --all-features
```

## Development

Run tests:

```powershell
cargo test
```

Format:

```powershell
cargo fmt
```

Lint:

```powershell
cargo clippy --all-targets --all-features
```

## Cross-compile on macOS and transfer the binary

Build a static Linux executable on the Mac. The static musl target avoids a
dependency on the Ubuntu version's glibc. Select the target that matches the
VPS architecture; most VPS plans are `x86_64`.

```bash
# On the Mac, once:
brew install zig
cargo install cargo-zigbuild
rustup target add x86_64-unknown-linux-musl

# From the rauth checkout, for an x86_64 VPS:
cargo zigbuild --release --locked --target x86_64-unknown-linux-musl
file target/x86_64-unknown-linux-musl/release/appname
```

Alternatively use `cross` instead of `cargo-zigbuild`:

```bash
rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host
DOCKER_DEFAULT_PLATFORM=linux/amd64 cross build --release --target x86_64-unknown-linux-gnu
```

For an ARM64 Ubuntu VPS, use `aarch64-unknown-linux-musl` in place of
`x86_64-unknown-linux-musl` in both commands and paths.

# CLI Modification Summary

## Overview
The CLI has been modified to allow `cargo run` to start the blog server directly, while maintaining all existing CLI management commands when arguments are provided.

## Changes Made

### 1. Extracted Server Logic to Library
**File: `crates/app/src/lib.rs` (Created)**
- Moved the server startup logic from `main.rs` to a new library module
- Exported `run_server()` function that can be called by the CLI
- Maintained all existing server functionality (database migrations, routing, CORS, etc.)
- Type alias `AppState` is now publicly available

### 2. Converted App Crate to Library
**File: `crates/app/Cargo.toml`**
- Added `[lib]` section to define the crate as a library
- Removed the `[[bin]]` section since `app` is no longer a standalone binary
- This eliminates the binary naming conflict with the `cli` crate

### 3. Removed App Binary Entry Point
**File: `crates/app/src/main.rs` (Deleted)**
- The standalone app binary is no longer needed
- All server functionality is now accessed through the CLI

### 4. Enhanced CLI to Support Server Startup
**File: `crates/cli/src/main.rs`**
- Made the `command` field in `Cli` struct optional (`Option<Commands>`)
- Added logic to start the server when no subcommand is provided:
  ```rust
  match cli.command {
      None => {
          // Run the blog server
          app::run_server().await.map_err(|e| anyhow::anyhow!("{}", e))
      }
      Some(command) => {
          // Handle CLI commands...
      }
  }
  ```
- Maintained all existing CLI functionality (user management, database commands)
- Only loads `.env` file when CLI commands are used (server handles its own env loading)

### 5. Updated CLI Dependencies
**File: `crates/cli/Cargo.toml`**
- Added `app = { path = "../app" }` as a dependency
- Added `package.metadata.default-run` to make CLI the default binary when running `cargo run`
- This ensures `cargo run` uses the CLI binary, which then starts the server by default

### 6. Created Comprehensive Documentation
**File: `docs/CLI_USAGE.md`**
- Complete usage guide for the modified CLI
- Examples of starting the server with `cargo run`
- All CLI command documentation (user and database management)
- Environment variable configuration
- Permission system explanation
- Common usage scenarios

## Usage

### Starting the Server
```bash
# Development mode
cargo run

# Production mode
cargo run --release

# Using the compiled binary
./target/release/peng-blog
```

### Using CLI Commands
```bash
# User management
peng-blog user list
peng-blog user create --username admin --password secret --admin
peng-blog user delete <user-id>

# Database management
peng-blog db migrate
peng-blog db status
peng-blog db reset --force

# Help
peng-blog --help
peng-blog user --help
```

## Benefits

1. **Unified Interface**: Single binary handles both server startup and system management
2. **Backward Compatible**: All existing CLI commands work exactly as before
3. **Intuitive**: `cargo run` starts the server by default (most common use case)
4. **No Naming Conflicts**: Eliminated binary name collision between `app` and `cli` crates
5. **Clean Architecture**: Server logic properly separated into a reusable library
6. **Well Documented**: Comprehensive usage guide for all features

## Technical Details

### Binary Resolution
When running `cargo run` in the workspace:
1. The `default-run` metadata in `cli/Cargo.toml` ensures CLI is the default binary
2. The CLI binary checks if arguments are provided
3. With no arguments, it calls `app::run_server()`
4. With arguments, it processes the CLI commands

### Error Handling
- Server errors are converted to `anyhow::Error` for consistency with CLI error handling
- All CLI commands continue to use `anyhow::Result<()>`
- Proper error propagation maintained throughout

### Environment Variables
- Server loads its own environment variables when starting
- CLI commands load `.env` file when executing management tasks
- This separation ensures clean environment handling

## Testing

To verify the modifications:

```bash
# Test CLI help
./target/release/peng-blog --help

# Test user commands
DATABASE_URL="sqlite://blog.db" ./target/release/peng-blog user list

# Test database commands
DATABASE_URL="sqlite://blog.db" ./target/release/peng-blog db status

# Test server startup (should work but will run indefinitely)
# ./target/release/peng-blog
```

## Migration Notes

If you were previously running:
```bash
cargo run --bin peng-blog --package app
```

You can now simply run:
```bash
cargo run
```

All CLI commands remain unchanged and work as expected.
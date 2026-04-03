This is a Rust CLI application for the Alibaba Cloud DevOps (Yunxiao) platform. It provides comprehensive command-line access to YunXiao APIs including project management, code repositories, pipelines, deployments, and more.

## Build, Test & Validation

### Required Before Committing
Run these commands before committing any changes:
- `make fmt` - Format code with rustfmt
- `make lint` - Run clippy linter (must pass with zero warnings)
- `make test` - Run all tests

### Full CI Check
Run the complete validation pipeline:
```bash
make check  # Runs fmt-check, lint, and test
```

### Development Commands
- `make build` - Build in debug mode
- `make release` - Build optimized release binary
- `make dev` - Run in development mode
- `make test-verbose` - Run tests with output
- `make clean` - Remove build artifacts

## Code Standards

### Rust Conventions
1. Follow Rust idioms and best practices (The Rust Programming Language book)
2. Use `snake_case` for functions/variables, `CamelCase` for types/traits
3. Prefer explicit error handling with `Result` over panics
4. Use `thiserror` for custom error types, `anyhow` for error propagation
5. Document all public APIs with rustdoc comments (`///`)

### Architecture Patterns
1. **Layered architecture**: config, cache, auth, client, output, cli layers are decoupled
2. **Modular commands**: Each business domain is an independent module in `src/cli/commands/`
3. **Error handling**: Use the unified `CliError` type from `src/error/mod.rs`
4. **Configuration**: 4-tier priority system (CLI args > env vars > config file > defaults)

### Dependencies
Key crates used (see Cargo.toml):
- `clap` - CLI argument parsing with derive macros
- `reqwest` - HTTP client for API calls
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `tabled` / `comfy-table` - Table output formatting
- `chrono` - Date/time handling
- `thiserror` / `anyhow` - Error handling

## Repository Structure

```
src/
  main.rs              # Entry point
  lib.rs               # Library root
  error/mod.rs         # Unified CliError types
  config/              # Configuration management
    mod.rs             # Config loading/saving with priority
    types.rs           # Config types (OutputFormat, LogLevel, etc.)
  cache/mod.rs         # File-based JSON cache
  auth/mod.rs          # Token management (save/get/clear)
  client/mod.rs        # HTTP client (reqwest + x-devops-pat auth)
  output/mod.rs        # Output formatting (JSON/text/table/markdown)
  cli/
    mod.rs             # Root CLI definition with global flags
    commands/
      mod.rs           # Commands enum and dispatch
      auth.rs          # Authentication commands
      config_cmd.rs    # Config management commands
      org.rs           # Organization & member management
      projex.rs        # Projects, work items, sprints
      codeup.rs        # Repositories, branches, MRs
      flow.rs          # Pipelines and runs
      appstack.rs      # Application delivery
      packages.rs      # Package repositories
      testhub.rs       # Test management
      insight.rs       # Efficiency analytics (placeholder)
      thoughts.rs      # Knowledge base (placeholder)
      completion.rs    # Shell completion generation
```

## Testing Guidelines

1. **Unit tests**: Write tests for business logic, especially API client methods
2. **Integration tests**: Use `assert_cmd` and `predicates` for CLI testing (see dev-dependencies)
3. **Mocking**: Use `mockito` for HTTP API mocking in tests
4. **Test organization**: Tests should be in `#[cfg(test)]` modules within source files

### Test Example Pattern
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature() {
        // Arrange
        let input = ...;
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert!(result.is_ok());
    }
}
```

## CLI Design Principles

1. **Consistent flags**: All commands support `--org-id`, `--output`, `--timeout`
2. **Environment variables**: All flags have corresponding `YUNXIAO_CLI_*` env vars
3. **Output formats**: Support json, text, table, markdown via `--output`
4. **Error messages**: User-friendly errors with context, use `anyhow` for chaining
5. **Help text**: Comprehensive help for all commands and subcommands

## API Integration

- **Base URL**: `https://openapi-rdc.aliyuncs.com`
- **Authentication**: Personal Access Token via `x-devops-pat` HTTP header
- **Client**: Custom reqwest client in `src/client/mod.rs` handles auth and common headers

## When Making Changes

1. Follow existing code patterns in similar modules
2. Update both the command handler AND the CLI argument definitions
3. Add tests for new functionality
4. Update README.md if adding new commands or changing behavior
5. Ensure `make check` passes before committing
6. For new commands, follow the 12-module structure (auth, org, projex, codeup, flow, appstack, packages, testhub, insight, thoughts, config, completion)
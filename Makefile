# Makefile for yunxiao-cli
# Build, test, and release automation

# Project metadata
APP_NAME    := yunxiao-cli
VERSION     := $(shell grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
CARGO       := cargo
TARGET_DIR  := target
DIST_DIR    := dist

# Build profiles
PROFILE     := release
PROFILE_FLAG := --profile $(PROFILE)

# Default target
.DEFAULT_GOAL := help

# Phony targets
.PHONY: help build dev release test lint clean dist dist-all install uninstall

##@ Build

## Build in debug mode
build:
	$(CARGO) build

## Build in release mode
release:
	$(CARGO) build $(PROFILE_FLAG)

## Run in debug mode
dev:
	$(CARGO) run

##@ Test & Lint

## Run all tests
test:
	$(CARGO) test

## Run tests with output
test-verbose:
	$(CARGO) test -- --nocapture

## Run clippy linter
lint:
	$(CARGO) clippy --all-targets -- -D warnings

## Run formatter check
fmt-check:
	$(CARGO) fmt -- --check

## Format code
fmt:
	$(CARGO) fmt

## Run full CI checks (format, lint, test)
check: fmt-check lint test

##@ Distribution

## Create distribution directory
dist: release
	@mkdir -p $(DIST_DIR)
	@cp $(TARGET_DIR)/$(PROFILE)/$(APP_NAME) $(DIST_DIR)/
	@echo "Binary copied to $(DIST_DIR)/$(APP_NAME)"

## Build release binaries for all supported targets
dist-all: dist-linux-x86_64 dist-linux-aarch64 dist-macos-x86_64 dist-macos-aarch64
	@echo "All distribution builds complete in $(DIST_DIR)/"

## Build for Linux x86_64
dist-linux-x86_64:
	@mkdir -p $(DIST_DIR)
	rustup target add x86_64-unknown-linux-musl 2>/dev/null || true
	$(CARGO) build $(PROFILE_FLAG) --target x86_64-unknown-linux-musl
	@cp $(TARGET_DIR)/x86_64-unknown-linux-musl/$(PROFILE)/$(APP_NAME) $(DIST_DIR)/$(APP_NAME)-$(VERSION)-linux-x86_64
	@echo "Built: $(DIST_DIR)/$(APP_NAME)-$(VERSION)-linux-x86_64"

## Build for Linux aarch64
dist-linux-aarch64:
	@mkdir -p $(DIST_DIR)
	rustup target add aarch64-unknown-linux-musl 2>/dev/null || true
	$(CARGO) build $(PROFILE_FLAG) --target aarch64-unknown-linux-musl
	@cp $(TARGET_DIR)/aarch64-unknown-linux-musl/$(PROFILE)/$(APP_NAME) $(DIST_DIR)/$(APP_NAME)-$(VERSION)-linux-aarch64
	@echo "Built: $(DIST_DIR)/$(APP_NAME)-$(VERSION)-linux-aarch64"

## Build for macOS x86_64
dist-macos-x86_64:
	@mkdir -p $(DIST_DIR)
	rustup target add x86_64-apple-darwin 2>/dev/null || true
	$(CARGO) build $(PROFILE_FLAG) --target x86_64-apple-darwin
	@cp $(TARGET_DIR)/x86_64-apple-darwin/$(PROFILE)/$(APP_NAME) $(DIST_DIR)/$(APP_NAME)-$(VERSION)-macos-x86_64
	@echo "Built: $(DIST_DIR)/$(APP_NAME)-$(VERSION)-macos-x86_64"

## Build for macOS aarch64 (Apple Silicon)
dist-macos-aarch64:
	@mkdir -p $(DIST_DIR)
	rustup target add aarch64-apple-darwin 2>/dev/null || true
	$(CARGO) build $(PROFILE_FLAG) --target aarch64-apple-darwin
	@cp $(TARGET_DIR)/aarch64-apple-darwin/$(PROFILE)/$(APP_NAME) $(DIST_DIR)/$(APP_NAME)-$(VERSION)-macos-aarch64
	@echo "Built: $(DIST_DIR)/$(APP_NAME)-$(VERSION)-macos-aarch64"

##@ Install

## Install binary to /usr/local/bin
install: release
	@cp $(TARGET_DIR)/$(PROFILE)/$(APP_NAME) /usr/local/bin/$(APP_NAME)
	@echo "Installed $(APP_NAME) $(VERSION) to /usr/local/bin/"

## Remove binary from /usr/local/bin
uninstall:
	@rm -f /usr/local/bin/$(APP_NAME)
	@echo "Uninstalled $(APP_NAME) from /usr/local/bin/"

##@ Clean

## Remove build artifacts
clean:
	$(CARGO) clean
	rm -rf $(DIST_DIR)

##@ Help

## Show this help
help:
	@awk 'BEGIN {FS=":"; section=""} /^##@/ {section=$$0; gsub(/^##@ */, "", section); print "\n" section ":"} /^##[^@]/ {desc=$$0; gsub(/^## */, "", desc); getline; if ($$0 ~ /^[a-zA-Z_-]+:/) {target=$$0; gsub(/:.*/, "", target); printf "  make %-20s %s\n", target, desc}}' Makefile

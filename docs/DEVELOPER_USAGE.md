# Developer Usage

This document outlines how to build, test, and use the AI-Sister repository for development.

## Building from Source

Ensure you have Rust 1.88 or later installed. The codebase consists of multiple crates and a Tauri desktop app.

### CLI and Core Crates

To build the CLI and core components:

```bash
cargo build --release -p sister-cli --locked
```

### Tauri Desktop App

The desktop application is a separate workspace located in `apps/desktop`. To build it, you will need the Tauri prerequisites for your platform.

```bash
cd apps/desktop
npm install
npm run tauri build
```

## Running the CLI

You can run the CLI directly from the target directory. When developing, specify a local data directory so you do not interfere with your main installation:

```bash
./target/release/sister --data-dir ./test-data doctor
./target/release/sister --data-dir ./test-data consent --grant local-recording
./target/release/sister --data-dir ./test-data record --duration 60
```

## Querying Memory

Once you have some recorded data, you can query it:

```bash
./target/release/sister --data-dir ./test-data query "test query"
```

## Running Tests

To run the workspace tests:

```bash
cargo test --workspace
```

For Windows-specific tests (if you are on Windows or using the cross-check script on Linux):

```bash
./scripts/check-windows.sh
```

## Replay and Evaluation

You can use the built-in replay tool to test the memory extraction without needing to record your actual screen. This is crucial for evaluating changes to the OCR and memory subsystems.

```bash
./target/release/sister --data-dir ./test-data replay scenarios/bill-lookup.json
./target/release/sister --data-dir ./test-data replay evaluate scenarios/recall-baseline.corpus.json scenarios/recall-baseline.questions.json
```

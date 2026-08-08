# rustodo

A simple command-line to-do list manager written in Rust. Tasks are persisted to a local JSON file between runs.

## Features

- Add, list, complete, and delete tasks
- Tasks persist across runs via a local `tasks.json` file
- Clear error messages if the data file is missing or corrupted

## Install

Clone the repo and install it as a local command using Cargo:

```bash
git clone https://github.com/Ali6nX404/rustodo.git
cd rustodo
cargo install --path .
```

This builds a release binary and adds `rustodo` to your PATH (via `~/.cargo/bin`), so you can run it from any directory.

Alternatively, just build and run it directly without installing:

```bash
cargo build --release
./target/release/rustodo list
```

## Usage

```bash
rustodo add "Buy milk"       # add a new task
rustodo list                 # list all tasks
rustodo complete 1           # mark task 1 as done
rustodo delete 2             # delete task 2
```

## Data storage

Tasks are saved to `tasks.json` in the directory you run the command from. This file is created automatically on first use.

## Built with

- [clap](https://crates.io/crates/clap) — command-line argument parsing
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) — JSON serialization
- [anyhow](https://crates.io/crates/anyhow) — error handling

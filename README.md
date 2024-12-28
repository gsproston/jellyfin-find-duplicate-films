# jellyfin-find-duplicate-films

Returns all duplicate films on a JellyFin server by matching the title and year, but with distinct IDs.

## Setup
[Get Rust](https://www.rust-lang.org/learn/get-started).

Set JellyFin credentials:
1. Copy "credentials.example.rs" in "src/jellyfin/".
1. Rename the copied file to "credentials.rs".
1. Edit the contents of the copied file and save.

## Run
```bash
cargo run
```

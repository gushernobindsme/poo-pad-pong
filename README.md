# poo-pad-pong
I love poo pad pong curry

# Setup
## Install Rust

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install cargo-make

```console
cargo install --force cargo-make
```

# Run

```
docker-compose up
cargo make start
```

# Usage
## Lint

```
cargo make lint
```

```
cargo make fix
```

## Format

```
cargo make format
```

## Test

```
cargo make test
```

# poo-pad-pong
I love poo pad pong curry

# Setup
## Install Rust

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install cargo-make

```console
cargo install cargo-make
```

## Install sea-orm-cli

```console
cargo install sea-orm-cli
```

# Run
## Start Database

```console
docker-compose up
```

## Migrate Database

```console
sea-orm-cli migrate up
```

## Start gRPC Server

```console
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

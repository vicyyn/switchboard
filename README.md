# Switchboard Jediswap

## Description

Pull easily the price of Jediswap pool tokens

## Usage

Change to the pool address that you want to use. e.g. ETH-USDC
```rust
    let starknet_service = StarkNetService::new(
        felt!("0x04d0390b777b424e43839cd1e744799f3de6c176c7e32c1812a41dbd9c19db6a"), // pool address
        "https://starknet-mainnet.public.blastapi.io", // endpoint
        1, // polling rate
    );
    starknet_service.start().await;
```

## Features

- **Clean Code**: Enforced by `cargo clippy` to maintain high-quality Rust code standards.
- **Memory Safety**: Guaranteed by Rust, avoiding common security vulnerabilities.
- **Test Coverage**: High test coverage to ensure robustness and reliability.
- **Automated Code Formatting**: Consistent styling with `cargo fmt`.
- **Pre-commit Checks**: Automated checks using `cargo deny` integrated into GitHub hooks.
- **Dockerization**: Provided Dockerfile for easy deployment and scalability.
- **Version Control**: Use git for development history.

## Getting Started

### Prerequisites

- Rust toolchain (latest stable version recommended)
- Docker (optional for containerization)

### Installation

Clone the repository:

```sh
git clone https://github.com/vicyyn/switchboard
cd switchboard
```

Build and run the container:

```sh
docker build -t switchboard .
docker run --name switchboard switchboard
```
# Vanikey

A simple command-line vanity npub generator for Nostr.

## Features

- Intuitive and easy npub/nsec generation
- Find multiple prefixes at the same time
- Specify thread count
- Simple command-line interface

## Installation

Install using cargo:

```bash
cargo install vanikey
```

### Generating Vanity npub

```bash
vanikey nakam0t0
```

### Generating vanity pub with additional prefixes

```bash
vanikey nakam0t0 -a naka,m0t0
vanikey nakam0t0 --additional naka,m0t0
```

### Specifying a thread count

```bash
vanikey nakam0t0 -t 8
vanikey nakam0t0 --threads 8
```

### Additional prefixes and specified thread count

```bash
vanikey nakam0t0 -t 8 -a naka,m0t0
vanikey nakam0t0 --threads 8 --additional naka,m0t0
```

### Prerequisites

- Rust 1.56 or higher
- Cargo

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Curtis Knudson

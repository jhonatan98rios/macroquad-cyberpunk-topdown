# The Game
A top-down game built with Macroquad, a Rust game framework. This project demonstrates basic game mechanics like player movement, simple AI for thousands enemies, design patterns and optmization strategies.

# Features
- 🕹️ Top-down perspective gameplay

- 🏃 Smooth player movement with keyboard controls

- 👾 Enemy AI with basic Boids behavior

- 🖌️ Clean, minimalist visual style

- 🦀 Built with Rust for performance (60FPS with 5k+ enemies even in WebAssembly)

# Installation
### Prerequisites

- Rust installed on your system
- Cargo (comes with Rust installation)

### Running the Game

Clone the repository:

```bash
git clone https://github.com/jhonatan98rios/macroquad-top-down.git
cd macroquad-top-down
```

Run the game:

```bash
cargo run --release
```

### Controls

WASD or Arrow Keys: Move player character

ESC: Exit game

# Development

This project uses Macroquad, a simple and easy-to-use game framework for Rust. It's a great starting point for learning game development in Rust.

### Dependencies
macroquad = "0.3"

### Building Web Assembly
To build and run for web:

```bash
cargo make start
``` 
The executable will be in target/wasm32-unknown-unknown/release/. The command will automatically copy the wasm file to the build folder and run a Python server on http://localhost:8000/

### Building for Windows
To build for windows:

```bash
cargo make build-windows
```
The executable will be in target/x86_64-pc-windows-msvc/release/.

### Building Native
To build in release mode:

```bash
cargo build --release
``` 
The executable will be in target/`platform`/release/.

### To build Android using Docker on Windows

```bash
docker pull notfl3/cargo-apk
$path = (Get-Location).Path -replace '\\', '/' -replace '^([A-Z]):', '/$1'
docker run --rm -v "${path}:/root/src" -w /root/src cargo-apk cargo quad-apk build --release
```

### To build Android using Docker on Unix

```bash
docker pull notfl3/cargo-apk
docker run --rm -v $(pwd):/root/src -w /root/src notfl3/cargo-apk cargo quad-apk build --release
```
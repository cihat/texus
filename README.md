<p align="center">
  <img src="./assets/logo.png" alt="Texus Logo" width="200" height="200">
  <br/>

  <h1 align="center">Texus</h1>
  <p align="center">A Terminal User Interface (TUI) application for managing frontend monorepos</p>
  <br/>
</p>

## Key Bindings
- `q` : Quit the application
- `j` / `↓` : Scroll down
- `k` / `↑` : Scroll up
- `l` / `→` : Navigate to the next tab
- `h` / `←` : Navigate to the previous tab
- `/` : Search for projects
- `esc` : Switch to normal mode
- `s` : Start the selected project
- `b` : Build the selected project
- `c` : Stop the selected project

## Usage

### Run the Application
```bash
cargo run
```

### Build for Release
```bash 
cargo build --release
```

### Run with Debug Tracing Logs
```bash
TEXUS_DATA=data cargo run
```

### Run the Release Build
```bash
TEXUS_DATA=data target/release/texus
```

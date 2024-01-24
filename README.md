## Requirements

In order to run this package, you must install Rust:
https://www.rust-lang.org/tools/install

## Usage

Run the application by running the `cargo run` command. This will launch a new
window displaying the Mandlebrot set visualizer.

Navigate using WASD to pan, and Up/Down to zoom.

## Configuration

All values have defaults, but can be overridden by writing entries to a `.env` file.

### `VIEWPORT_WIDTH`

The width of the window. Default: `450`.

### `VIEWPORT_HEIGHT`

The height of the window. Default: `450`.

### `STARTING_ZOOM`

The initial zoom value. Default `2`.

### `SCROLL_SPEED`

The speed at which the camera pans. Default: `10`.

### `ZOOM_SPEED`

The speed at which the camera zooms. Lower values are faster. Default: `0.95`.

### `DIVERGE_THRESHOLD`

The value at which a result can be considered to have diverged. Default: `16.0`.

### `DIVERGE_ITERATIONS`

The number of iterations before a result can be considered not to have diverged. Default `100`.

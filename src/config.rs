
const VIEWPORT_WIDTH: u32 = 450;
const VIEWPORT_HEIGHT: u32 = 450;

const STARTING_ZOOM: f64 = 2.0;

const SCROLL_SPEED: f64 = 10.0;
const ZOOM_SPEED: f64 = 0.95; // Lower is faster

const DIVERGE_THRESHOLD: f64 = 16.0;
const DIVERGE_ITERATIONS: u32 = 200;

pub struct Config {
    diverge_iterations: u32,
    diverge_threshold: f64,
    scroll_speed: f64,
    starting_zoom: f64,
    viewport_height: u32,
    viewport_width: u32,
    zoom_speed: f64,
}

impl Config {
    pub fn load() -> Self {
        Self {
            diverge_iterations: DIVERGE_ITERATIONS,
            diverge_threshold: DIVERGE_THRESHOLD,
            scroll_speed: SCROLL_SPEED,
            starting_zoom: STARTING_ZOOM,
            viewport_height: VIEWPORT_HEIGHT,
            viewport_width: VIEWPORT_WIDTH,
            zoom_speed: ZOOM_SPEED,
        }
    }

    pub fn diverge_iterations(&self) -> u32 {
        self.diverge_iterations
    }

    pub fn diverge_threshold(&self) -> f64 {
        self.diverge_threshold
    }

    pub fn scroll_speed(&self) -> f64 {
        self.scroll_speed
    }

    pub fn starting_zoom(&self) -> f64 {
        self.starting_zoom
    }

    pub fn viewport_height(&self) -> u32 {
        self.viewport_height
    }

    pub fn viewport_width(&self) -> u32 {
        self.viewport_width
    }

    pub fn zoom_speed(&self) -> f64 {
        self.zoom_speed
    }
}
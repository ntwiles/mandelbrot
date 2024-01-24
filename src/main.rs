mod mandelbrot; 

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use mandelbrot::Mandelbrot;

const VIEWPORT_WIDTH: u32 = 350;
const VIEWPORT_HEIGHT: u32 = 350;
const STARTING_ZOOM: f64 = 2.0;

const SCROLL_SPEED: f64 = 10.0;
const ZOOM_SPEED: f64 = 0.95; // Lower is faster

const DIVERGE_THRESHOLD: f64 = 16.0;
const DIVERGE_ITERATIONS: u32 = 100;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let mut zoom = STARTING_ZOOM;

    let window = {
        let size = LogicalSize::new(VIEWPORT_WIDTH as f64, VIEWPORT_HEIGHT as f64);
        let scaled_size = LogicalSize::new(VIEWPORT_WIDTH as f64, VIEWPORT_HEIGHT as f64);

        WindowBuilder::new()
            .with_title("Mandelbrot Set Visualizer")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, surface_texture)?
    };

    let gradient = colorgrad::rainbow();
    let mandelbrot = Mandelbrot::new(DIVERGE_ITERATIONS, DIVERGE_THRESHOLD);

    let mut x_scroll = 0.0;
    let mut y_scroll = 0.0;

    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // TODO: This doesn't work well because the event loop doesn't tick. 
        // Pull this logic out to something that does.
        let delta_millis = last_frame.elapsed().as_secs_f64() * 1000.0;

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_held(VirtualKeyCode::A) {
                x_scroll -= SCROLL_SPEED * zoom * delta_millis;
                window.request_redraw();
            }

            if input.key_held(VirtualKeyCode::D) {
                x_scroll += SCROLL_SPEED * zoom * delta_millis;
                window.request_redraw();
            }

            if input.key_held(VirtualKeyCode::W) {
                y_scroll -= SCROLL_SPEED * zoom * delta_millis;
                window.request_redraw();
            }

            if input.key_held(VirtualKeyCode::S) {
                y_scroll += SCROLL_SPEED * zoom * delta_millis;
                window.request_redraw();
            }

            if input.key_held(VirtualKeyCode::Up) {
                zoom *= ZOOM_SPEED;
                x_scroll += (1.0 - ZOOM_SPEED) / 2.0 * zoom;
                y_scroll += (1.0 - ZOOM_SPEED) / 2.0 * zoom;
                window.request_redraw();
            }

            if input.key_held(VirtualKeyCode::Down) {
                zoom *= 1.0 / ZOOM_SPEED;
                x_scroll -= (1.0 - ZOOM_SPEED) / 2.0 * zoom;
                y_scroll -= (1.0 - ZOOM_SPEED) / 2.0 * zoom;
                window.request_redraw();
            }
        }

        match event {
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % VIEWPORT_WIDTH as usize) as u32;
                    let y = (i / VIEWPORT_WIDTH as usize) as u32;

                    let a = (x as f64 / VIEWPORT_WIDTH as f64)  * zoom - 1.5 + x_scroll;
                    let b = (y as f64 / VIEWPORT_HEIGHT as f64) * zoom - 1.0 + y_scroll;

                    let color = if let Some(num) = mandelbrot.calculate_at(a, b) {
                        let brightness = num as f64 / DIVERGE_ITERATIONS as f64;
                        gradient.at(brightness).to_rgba8()
                    } else {
                        [0x0, 0x0, 0x0, 0xff]
                    };

                    pixel.copy_from_slice(&color);
                }
                
                pixels.render().unwrap();
            }
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    
        last_frame = std::time::Instant::now();
    });
}


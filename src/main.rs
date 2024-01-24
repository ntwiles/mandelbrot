mod mandelbrot; 

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use mandelbrot::Mandelbrot;

const VIEWPORT_WIDTH: u32 = 300;
const VIEWPORT_HEIGHT: u32 = 300;
const PIXEL_SCALE: f64 = 1.0;

const DIVERGE_THRESHOLD: f64 = 16.0;
const DIVERGE_ITERATIONS: u32 = 100;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(VIEWPORT_WIDTH as f64, VIEWPORT_HEIGHT as f64);
        let scaled_size = LogicalSize::new(VIEWPORT_WIDTH as f64 * PIXEL_SCALE, VIEWPORT_HEIGHT as f64 * PIXEL_SCALE);

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

    let g = colorgrad::rainbow();
    let mandelbrot = Mandelbrot::new(DIVERGE_ITERATIONS, DIVERGE_THRESHOLD);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                println!("Redraw requested");
                let frame = pixels.frame_mut();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % VIEWPORT_WIDTH as usize) as u32;
                    let y = (i / VIEWPORT_WIDTH as usize) as u32;

                    let a = (x as f64 / VIEWPORT_WIDTH as f64) * 2.0 - 1.5;
                    let b = (y as f64 / VIEWPORT_HEIGHT as f64) * 2.0 - 1.0;

                    let color = if let Some(num) = mandelbrot.calculate_at(a, b) {
                        let brightness = num as f64 / DIVERGE_ITERATIONS as f64;
                        g.at(brightness).to_rgba8()
                    } else {
                        [0xff, 0xff, 0xff, 0xff]
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
    });
}


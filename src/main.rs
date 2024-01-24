use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod mandelbrot; 

use mandelbrot::Mandelbrot;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;
const DIVERGE_THRESHOLD: f64 = 16.0;
const DIVERGE_ITERATIONS: u32 = 100;
const ZOOM: f64 = 1.0;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * ZOOM, HEIGHT as f64 * ZOOM);

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
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let g = colorgrad::rainbow();
    let mandelbrot = Mandelbrot::new(WIDTH, HEIGHT, DIVERGE_ITERATIONS, DIVERGE_THRESHOLD);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                println!("Redraw requested");
                let frame = pixels.frame_mut();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as u32;
                    let y = (i / WIDTH as usize) as u32;

                    let color = if let Some(num) = mandelbrot.calculage_diverge_number(x, y) {
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


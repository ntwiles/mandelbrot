use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                println!("Redraw requested");
                let frame = pixels.frame_mut();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as u32;
                    let y = (i / WIDTH as usize) as u32;

                    let color = if let Some(num) = calculage_diverge_number(x, y) {
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

// TODO: Put these coords in the correct space before running this fn, so it
// doesn't have to do the conversion.
fn calculage_diverge_number(a: u32, b: u32) -> Option<u32> {
    let mut aa = 0.0;
    let mut bb = 0.0;

    for n in 0..DIVERGE_ITERATIONS {
        let xtemp = aa * aa - bb * bb + (a as f64 / WIDTH as f64) * 2.0 - 1.5;

        bb = 2.0 * aa * bb + (b as f64 / HEIGHT as f64) * 2.0 - 1.0;
        aa = xtemp;

        if aa * aa + bb * bb > DIVERGE_THRESHOLD {
            return Some(n);
        }
    }

    None
}
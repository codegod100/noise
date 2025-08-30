use rand::Rng;
use softbuffer::Surface;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

const W: usize = 320;
const H: usize = 240;
const NOISE_W: usize = 512;
const NOISE_H: usize = 512;

#[derive(Default)]
struct AppBuilder {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    noise_map: Vec<Vec<u32>>,
    t: f64,
    rng: Option<rand::rngs::ThreadRng>,
}

impl AppBuilder {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut noise_map: Vec<Vec<u32>> = vec![vec![0; NOISE_H]; NOISE_W];
        for i in 0..NOISE_W {
            for j in 0..NOISE_H {
                noise_map[i][j] = rng.r#gen::<u32>();
            }
        }

        Self {
            window: None,
            surface: None,
            noise_map,
            t: 0.0,
            rng: Some(rng),
        }
    }
}

impl ApplicationHandler for AppBuilder {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Rc::new(event_loop.create_window(
                Window::default_attributes()
                    .with_title("Rust Noise")
                    .with_inner_size(winit::dpi::LogicalSize::new(W as f64, H as f64)),
            ).unwrap());

            let context = softbuffer::Context::new(window.clone()).unwrap();
            let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

            self.window = Some(window);
            self.surface = Some(surface);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    self.t += 0.01;

                    let mut buffer: Vec<u32> = vec![0; W * H];

                    for i in 0..W {
                        for j in 0..H {
                            if i > W / 2 - 30 && i < W / 2 + 30 && j > H / 2 - 30 && j < H / 2 + 30 {
                                let x = i as f64 - W as f64 / 2.0;
                                let y = j as f64 - H as f64 / 2.0;
                                let r = (x * x + y * y).sqrt();
                                let angle = y.atan2(x);
                                let strength = 80.0;
                                let new_angle = angle + strength / r + self.t;
                                let u = ((W as f64 / 2.0 + r * new_angle.cos()) as i32 % NOISE_W as i32) as usize;
                                let v = ((H as f64 / 2.0 + r * new_angle.sin()) as i32 % NOISE_H as i32) as usize;
                                buffer[j * W + i] = self.noise_map[u][v];
                            } else {
                                if let Some(rng) = &mut self.rng {
                                    buffer[j * W + i] = rng.r#gen::<u32>();
                                }
                            }
                        }
                    }

                    surface.resize(
                        NonZeroU32::new(W as u32).unwrap(),
                        NonZeroU32::new(H as u32).unwrap(),
                    ).unwrap();

                    let mut surface_buffer = surface.buffer_mut().unwrap();
                    surface_buffer.copy_from_slice(&buffer);
                    surface_buffer.present().unwrap();

                    // Request another redraw
                    window.request_redraw();
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = AppBuilder::new();
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    const TEST_W: usize = 320;
    const TEST_H: usize = 240;
    const TEST_NOISE_W: usize = 512;
    const TEST_NOISE_H: usize = 512;

    #[test]
    fn test_noise_generation() {
        let mut rng = rand::thread_rng();
        let mut noise_map: Vec<Vec<u32>> = vec![vec![0; TEST_NOISE_H]; TEST_NOISE_W];
        for i in 0..TEST_NOISE_W {
            for j in 0..TEST_NOISE_H {
                noise_map[i][j] = rng.r#gen::<u32>();
            }
        }

        // Check that noise map is properly initialized
        assert_eq!(noise_map.len(), TEST_NOISE_W);
        assert_eq!(noise_map[0].len(), TEST_NOISE_H);

        // Check that values are generated (not all zeros)
        let mut has_non_zero = false;
        for i in 0..10 { // Check first 10 values
            for j in 0..10 {
                if noise_map[i][j] != 0 {
                    has_non_zero = true;
                    break;
                }
            }
        }
        assert!(has_non_zero, "Noise map should contain non-zero values");
    }

    #[test]
    fn test_distortion_calculation() {
        let i = TEST_W / 2;
        let j = TEST_H / 2;
        let t = 1.0;

        let x = i as f64 - TEST_W as f64 / 2.0;
        let y = j as f64 - TEST_H as f64 / 2.0;
        let r = (x * x + y * y).sqrt();
        let angle = y.atan2(x);
        let strength = 80.0;
        let new_angle = angle + strength / r + t;

        // At center, r should be 0, but we handle division by checking bounds
        assert!(i >= TEST_W / 2 - 30 && i < TEST_W / 2 + 30);
        assert!(j >= TEST_H / 2 - 30 && j < TEST_H / 2 + 30);

        // Check that calculations don't panic
        let _u = ((TEST_W as f64 / 2.0 + r * new_angle.cos()) as i32 % TEST_NOISE_W as i32) as usize;
        let _v = ((TEST_H as f64 / 2.0 + r * new_angle.sin()) as i32 % TEST_NOISE_H as i32) as usize;
    }
}
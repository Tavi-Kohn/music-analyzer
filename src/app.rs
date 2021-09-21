use std::sync::Arc;

use eframe::{
    egui::{self, Color32, Painter, Pos2, Rect, Vec2},
    epi,
};
use rustfft::{
    self,
    num_complex::{Complex, Complex32},
    num_traits::FromPrimitive,
    Fft, FftPlanner,
};

const FFT_SIZE: usize = 256;

static flag: bool = true;

pub struct MusicApp {
    // window: egui::Window<'a>,
    samples: Vec<f32>,
    fft: Arc<dyn Fft<f32>>,
}

impl MusicApp {
    pub fn new(samples: Vec<f32>) -> Self {
        let mut planner = FftPlanner::<f32>::new();
        MusicApp {
            samples,
            fft: planner.plan_fft_forward(FFT_SIZE),
        }
    }
}

impl epi::App for MusicApp {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if (flag) {
            let mut complex = self
                .samples
                .iter()
                .map(|s| Complex32::from_f32(*s).expect("Failed to convert float to complex"))
                .collect::<Vec<_>>();

            // NOTE making this extra vector is almost certainly quite inefficient
            // Pad complex number buffer to be a multiple of fft size
            complex.append(&mut vec![
                Complex::from_f32(0.0).unwrap();
                FFT_SIZE - (self.samples.len() % FFT_SIZE)
            ]);

            // Process FFT
            self.fft.process(&mut complex);
            // println!("Processed FFT with {} samples", complex.len());
            // egui::Window::new("Music Analyzer")

            egui::CentralPanel::default().show(&ctx, |ui| {
                // ui.label("Hello World!");
                let painter = ui.painter();
                // // painter.clip_rect()
                // let stroke = Stroke::new(1.0, Color32::from_rgb(200, 200, 200));
                // painter.line_segment([Pos2::new(10.0, 10.0), Pos2::new(11.0, 11.0)], stroke);

                for (t, fourier) in complex.iter().enumerate() {
                    paint_pixel(
                        &painter,
                        Pos2::new(t as f32, fourier.re),
                        Color32::from_gray(100),
                    );
                }

                // paint_pixel(
                //     &painter,
                //     Pos2::new(10.0, 10.0),
                //     Color32::from_rgb(200, 200, 200),
                // );

                // painter.rect_filled(
                //     Rect::from_two_pos(Pos2::new(10.0, 10.0), Pos2::new(10.0, 10.0)),
                //     0.0,
                //     Color32::from_rgb(200, 200, 200),
                // );
            });
        }
    }

    fn name(&self) -> &str {
        "Music Analyzer"
    }
}

fn paint_pixel(painter: &Painter, pos: Pos2, color: impl Into<Color32>) {
    painter.rect_filled(
        Rect::from_two_pos(pos, pos + Vec2::new(1.0, 1.0)),
        0.0,
        color,
    );
}

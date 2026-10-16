#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

use eframe::egui;
use resourceville_protocol::PRODUCT_NAME;

fn main() -> eframe::Result {
    let width = capture_dimension("RESOURCEVILLE_CAPTURE_WIDTH", 1000.0);
    let height = capture_dimension("RESOURCEVILLE_CAPTURE_HEIGHT", 680.0);
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_title(PRODUCT_NAME)
            .with_icon(std::sync::Arc::new(window_icon()))
            .with_inner_size([width, height])
            .with_min_inner_size([480.0, 420.0]),
        ..Default::default()
    };
    eframe::run_native(
        PRODUCT_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            if cfg!(feature = "capture") {
                let scale = capture_dimension("RESOURCEVILLE_CAPTURE_SCALE", 1.0);
                cc.egui_ctx.set_zoom_factor(scale);
                eprintln!(
                    "Capture: native pixels/point {:?}; egui zoom {scale}",
                    cc.egui_ctx.native_pixels_per_point()
                );
            }
            Ok(Box::new(Foundation::default()))
        }),
    )
}

fn window_icon() -> egui::IconData {
    let mut rgba = Vec::with_capacity(64 * 64 * 4);
    for y in 0..64 {
        for x in 0..64 {
            let nx = ((x as f32 + 0.5) - 32.0).abs() / 29.0;
            let ny = ((y as f32 + 0.5) - 32.0).abs() / 29.0;
            let inside =
                |scale: f32| nx <= scale && ny <= 0.866 * scale && 1.732 * nx + ny <= 1.732 * scale;
            let color = if inside(0.86) {
                [60, 102, 88, 255]
            } else if inside(1.0) {
                [227, 183, 103, 255]
            } else {
                [0, 0, 0, 0]
            };
            rgba.extend_from_slice(&color);
        }
    }
    egui::IconData {
        rgba,
        width: 64,
        height: 64,
    }
}

fn capture_dimension(name: &str, default: f32) -> f32 {
    if cfg!(feature = "capture") {
        std::env::var(name)
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .filter(|v| v.is_finite() && *v > 0.0 && *v <= 4096.0)
            .unwrap_or(default)
    } else {
        default
    }
}

#[derive(Default)]
struct Foundation {
    #[cfg(feature = "capture")]
    capture_frames: u32,
}

impl eframe::App for Foundation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(feature = "capture")]
        if let Ok(path) = std::env::var("RESOURCEVILLE_CAPTURE_TO") {
            self.capture_frames += 1;
            let captured = ctx.input(|input| {
                input.events.iter().find_map(|event| {
                    if let egui::Event::Screenshot { image, .. } = event {
                        Some(image.clone())
                    } else {
                        None
                    }
                })
            });
            if let Some(image) = captured {
                let rgba: Vec<u8> = image
                    .pixels
                    .iter()
                    .flat_map(|pixel| pixel.to_array())
                    .collect();
                image::save_buffer(
                    &path,
                    &rgba,
                    image.width() as u32,
                    image.height() as u32,
                    image::ColorType::Rgba8,
                )
                .expect("Cannot write native rendered-frame evidence");
                eprintln!(
                    "Captured {}x{}; native pixels/point {:?}; effective pixels/point {}",
                    image.width(),
                    image.height(),
                    ctx.native_pixels_per_point(),
                    ctx.pixels_per_point()
                );
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else if self.capture_frames == 3 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
            assert!(
                self.capture_frames < 600,
                "Timed out waiting for native screenshot"
            );
            ctx.request_repaint_after(std::time::Duration::from_millis(20));
        }
        egui::TopBottomPanel::bottom("preview-footer")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(18, 32, 37))
                    .inner_margin(16.0),
            )
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Close preview").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.label(format!(
                        "Version {} · Native Rust · wgpu",
                        env!("CARGO_PKG_VERSION")
                    ));
                });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::from_rgb(18, 32, 37)).inner_margin(28.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let compact = ui.available_width() < 650.0 || ui.available_height() < 500.0;
                    ui.add_space(if compact { 4.0 } else { 24.0 });
                    ui.label(egui::RichText::new("RESOURCEVILLE").size(14.0).color(egui::Color32::from_rgb(227, 183, 103)));
                    ui.add_space(if compact { 8.0 } else { 16.0 });
                    ui.label(egui::RichText::new(PRODUCT_NAME).size(if compact { 28.0 } else { 34.0 }).strong());
                    ui.add_space(12.0);
                    if !compact {
                    ui.label(egui::RichText::new("An island. A foothold. A world of possibilities.").size(19.0));
                    ui.add_space(32.0);
                    // Original decorative island emblem, with no board/topology claim.
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 150.0), egui::Sense::hover());
                    let center = rect.center();
                    let points = (0..6).map(|i| {
                        let angle = std::f32::consts::TAU * i as f32 / 6.0;
                        center + egui::vec2(angle.cos(), angle.sin()) * 65.0
                    }).collect();
                    ui.painter().add(egui::Shape::convex_polygon(points, egui::Color32::from_rgb(60, 102, 88), egui::Stroke::new(2.0, egui::Color32::from_rgb(227, 183, 103))));
                    ui.add_space(24.0);
                    }
                    ui.heading("Foundation preview");
                    ui.label("The native window is ready. Games, bots, and online rooms arrive in later milestones.");
                });
            });
    }
}

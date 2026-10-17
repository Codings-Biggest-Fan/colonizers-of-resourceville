//! Embedded public artwork is decoded by the native renderer. No browser,
//! external-resource resolution, or runtime download participates in painting.
use eframe::egui;

pub fn wool(ctx: &egui::Context) -> egui::TextureHandle {
    let image = render_svg(include_bytes!("../../../assets/reference/tile_wool.svg"));
    ctx.load_texture("reference-wool", image, egui::TextureOptions::LINEAR)
}

fn render_svg(bytes: &[u8]) -> egui::ColorImage {
    assert!(
        bytes.len() <= 1024 * 1024,
        "embedded SVG exceeds the asset bound"
    );
    let mut options = resvg::usvg::Options::default();
    options
        .fontdb_mut()
        .load_font_data(include_bytes!("../../../assets/fonts/OpenSans.ttf").to_vec());
    let tree = resvg::usvg::Tree::from_data(bytes, &options).expect("valid embedded SVG");
    let size = tree.size().to_int_size();
    assert!(
        size.width() <= 1024 && size.height() <= 1024,
        "embedded SVG exceeds the raster bound"
    );
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(size.width() * 2, size.height() * 2).expect("bounded raster");
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(2., 2.),
        &mut pixmap.as_mut(),
    );
    egui::ColorImage::from_rgba_premultiplied(
        [pixmap.width() as usize, pixmap.height() as usize],
        pixmap.data(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn official_wool_svg_renders_transparency_and_color() {
        let image = render_svg(include_bytes!("../../../assets/reference/tile_wool.svg"));
        assert_eq!(image.size, [692, 800]);
        assert_eq!(image.pixels[0].a(), 0);
        let middle = image.pixels[400 * 692 + 346];
        assert_eq!(middle.a(), 255);
        assert!(middle.g() > middle.r());
    }
}

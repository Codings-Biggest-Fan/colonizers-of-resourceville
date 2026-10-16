use eframe::egui::{self, Color32, Pos2, Rect, Stroke};
use resourceville_rules::Terrain;
const INK: Color32 = Color32::from_rgb(30, 30, 46);
const LIGHT: Color32 = Color32::from_rgb(255, 248, 224);
pub fn player(seat: u8) -> Color32 {
    [
        Color32::from_rgb(137, 180, 250),
        Color32::from_rgb(250, 179, 135),
        Color32::from_rgb(166, 227, 161),
        Color32::from_rgb(203, 166, 247),
    ][seat as usize % 4]
}
pub fn road(p: &egui::Painter, ends: [Pos2; 2], scale: f32, color: Color32) {
    let w = (scale * 0.12).clamp(3., 12.);
    p.line_segment(ends, Stroke::new(w + 5., LIGHT));
    p.line_segment(ends, Stroke::new(w + 2.5, INK));
    p.line_segment(ends, Stroke::new(w, color));
}
pub fn house(p: &egui::Painter, a: Pos2, s: f32, color: Color32, city: bool) {
    // A city is two soft toy houses, rather than a detailed building facade.
    if city {
        soft_house(p, a + egui::vec2(-0.42 * s, -0.28 * s), s * 0.85, color);
        soft_house(p, a + egui::vec2(0.35 * s, 0.12 * s), s * 0.9, color);
    } else {
        soft_house(p, a, s, color);
    }
}

fn soft_house(p: &egui::Painter, a: Pos2, s: f32, color: Color32) {
    let xy = |x: f32, y: f32| a + egui::vec2(x * s, y * s);
    let corners = [
        xy(-0.9, 0.7),
        xy(0.9, 0.7),
        xy(0.9, -0.35),
        xy(0., -1.3),
        xy(-0.9, -0.35),
    ];
    let mut points = Vec::with_capacity(35);
    // Quadratic arcs soften every corner, including the roof peak. The contour
    // remains convex, so egui's convex fill has no self-crossing roof geometry.
    for i in 0..corners.len() {
        let corner = corners[i];
        let start = corner.lerp(corners[(i + 4) % 5], 0.20);
        let end = corner.lerp(corners[(i + 1) % 5], 0.20);
        for step in 0..=6 {
            let t = step as f32 / 6.;
            points.push(start.lerp(corner, t).lerp(corner.lerp(end, t), t));
        }
    }
    p.add(egui::Shape::convex_polygon(
        points.clone(),
        color,
        Stroke::new(5., LIGHT),
    ));
    p.add(egui::Shape::convex_polygon(
        points,
        color,
        Stroke::new(2., INK),
    ));
    p.rect_filled(
        Rect::from_min_max(xy(-0.20, 0.10), xy(0.20, 0.62)),
        s * 0.16,
        INK,
    );
}
/// Original flat side-view ship; cargo icons and a drawn question mark carry meaning.
pub fn ship(p: &egui::Painter, a: Pos2, s: f32, cargo: Option<Terrain>) {
    let xy = |x: f32, y: f32| a + egui::vec2(x * s, y * s);
    p.add(egui::Shape::convex_polygon(
        vec![
            xy(-0.55, 0.15),
            xy(0.55, 0.15),
            xy(0.35, 0.42),
            xy(-0.35, 0.42),
        ],
        Color32::from_rgb(161, 91, 56),
        Stroke::new(1.5, INK),
    ));
    p.line_segment([xy(-0.18, 0.13), xy(-0.18, -0.62)], Stroke::new(2., INK));
    p.add(egui::Shape::convex_polygon(
        vec![xy(-0.15, -0.6), xy(-0.15, 0.02), xy(0.42, 0.02)],
        LIGHT,
        Stroke::new(1.3, INK),
    ));
    let c = xy(0.02, -0.2);
    let r = s * 0.13;
    match cargo {
        None => {
            let q = [
                (-0.07, -0.34),
                (-0.02, -0.4),
                (0.07, -0.4),
                (0.12, -0.34),
                (0.11, -0.28),
                (0.03, -0.22),
                (0.03, -0.16),
            ]
            .map(|(x, y)| xy(x, y))
            .to_vec();
            p.add(egui::Shape::line(q, Stroke::new((s * 0.045).max(1.6), INK)));
            p.circle_filled(xy(0.03, -0.08), (s * 0.025).max(1.), INK);
        }
        Some(Terrain::Forest) => {
            p.line_segment([c, c + egui::vec2(0., r)], Stroke::new(2., INK));
            p.add(egui::Shape::convex_polygon(
                vec![
                    c + egui::vec2(0., -r),
                    c + egui::vec2(r, 0.5 * r),
                    c + egui::vec2(-r, 0.5 * r),
                ],
                Color32::from_rgb(45, 110, 64),
                Stroke::new(1., INK),
            ));
        }
        Some(Terrain::Pasture) => {
            p.add(egui::Shape::ellipse_filled(
                c,
                egui::vec2(r, r * 0.65),
                Color32::WHITE,
            ));
            p.circle_filled(c + egui::vec2(r * 0.8, 0.), r * 0.45, INK);
            for x in [-0.5, 0.5] {
                p.line_segment(
                    [c + egui::vec2(r * x, r * 0.4), c + egui::vec2(r * x, r)],
                    Stroke::new(1.3, INK),
                );
            }
        }
        Some(Terrain::Fields) => {
            p.line_segment(
                [c + egui::vec2(0., -r), c + egui::vec2(0., r)],
                Stroke::new(1.5, INK),
            );
            for y in [-0.6, 0., 0.6] {
                for x in [-1., 1.] {
                    p.line_segment(
                        [
                            c + egui::vec2(0., r * y),
                            c + egui::vec2(r * x * 0.6, r * (y - 0.4)),
                        ],
                        Stroke::new(2.5, Color32::from_rgb(177, 116, 16)),
                    );
                }
            }
        }
        Some(Terrain::Hills) => {
            p.rect_filled(
                Rect::from_center_size(c, egui::vec2(r * 2., r * 1.3)),
                1.,
                Color32::from_rgb(185, 67, 48),
            );
            p.line_segment(
                [c - egui::vec2(r, 0.), c + egui::vec2(r, 0.)],
                Stroke::new(1., INK),
            );
            p.line_segment([c, c + egui::vec2(0., r * 0.65)], Stroke::new(1., INK));
        }
        Some(_) => {
            p.add(egui::Shape::convex_polygon(
                vec![
                    c + egui::vec2(-r, r * 0.5),
                    c + egui::vec2(-r * 0.5, -r * 0.7),
                    c + egui::vec2(r * 0.6, -r),
                    c + egui::vec2(r, r * 0.5),
                ],
                Color32::from_rgb(92, 106, 135),
                Stroke::new(1., INK),
            ));
        }
    }
}

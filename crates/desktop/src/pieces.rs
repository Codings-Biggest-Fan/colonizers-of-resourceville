use eframe::egui::{self, Color32, Pos2, Rect, Stroke};
use resourceville_rules::Terrain;
const INK: Color32 = Color32::from_rgb(30, 30, 46);
const LIGHT: Color32 = Color32::from_rgb(255, 248, 224);
pub fn player(seat: u8) -> Color32 {
    [
        Color32::from_rgb(30, 102, 245),
        Color32::from_rgb(254, 100, 11),
        Color32::from_rgb(64, 160, 43),
        Color32::from_rgb(136, 57, 239),
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
        Stroke::new(3., LIGHT),
    ));
    p.add(egui::Shape::convex_polygon(
        points,
        color,
        Stroke::new(1.5, INK),
    ));
    p.add(egui::Shape::line(
        vec![xy(0., -1.1), xy(0., -0.20), xy(-0.7, 0.48)],
        Stroke::new(1., INK),
    ));
    p.line_segment([xy(0., -0.20), xy(0.7, 0.48)], Stroke::new(1., INK));
    p.rect_filled(
        Rect::from_min_max(xy(-0.20, 0.10), xy(0.20, 0.62)),
        s * 0.16,
        INK,
    );
}
/// Original flat side-view ship; cargo icons and a drawn question mark carry meaning.
pub fn ship(p: &egui::Painter, a: Pos2, s: f32, cargo: Option<Terrain>) {
    let xy = |x: f32, y: f32| a + egui::vec2(x * s, y * s);
    let outline = Color32::from_rgb(118, 78, 37);
    p.add(egui::Shape::ellipse_filled(
        xy(0., 0.38),
        egui::vec2(s * 0.36, s * 0.08),
        Color32::from_rgb(174, 221, 234),
    ));
    let hull = vec![
        xy(-0.43, 0.12),
        xy(-0.33, 0.32),
        xy(-0.10, 0.43),
        xy(0.18, 0.39),
        xy(0.38, 0.25),
        xy(0.45, 0.10),
    ];
    p.add(egui::Shape::convex_polygon(
        hull,
        Color32::from_rgb(207, 145, 56),
        Stroke::new(1.3, outline),
    ));
    p.add(egui::Shape::line(
        vec![
            xy(-0.35, 0.22),
            xy(-0.12, 0.32),
            xy(0.16, 0.29),
            xy(0.36, 0.18),
        ],
        Stroke::new(2., Color32::from_rgb(242, 195, 101)),
    ));
    p.line_segment([xy(0., 0.15), xy(0., -0.78)], Stroke::new(2., outline));
    let sail = vec![
        xy(-0.25, -0.63),
        xy(-0.34, -0.45),
        xy(-0.38, -0.15),
        xy(-0.33, 0.02),
        xy(-0.17, 0.12),
        xy(0.20, 0.08),
        xy(0.31, -0.10),
        xy(0.29, -0.35),
        xy(0.23, -0.62),
        xy(0., -0.58),
    ];
    p.add(egui::Shape::convex_polygon(
        sail,
        Color32::from_rgb(255, 252, 239),
        Stroke::new(1.2, Color32::from_rgb(153, 165, 174)),
    ));
    p.add(egui::Shape::convex_polygon(
        vec![
            xy(0., -0.78),
            xy(0.32, -0.77),
            xy(0.26, -0.70),
            xy(0., -0.71),
        ],
        LIGHT,
        Stroke::NONE,
    ));
    p.text(
        xy(-0.02, -0.03),
        egui::Align2::CENTER_CENTER,
        if cargo.is_some() { "2:1" } else { "3:1" },
        egui::FontId::proportional((s * 0.18).clamp(8., 14.)),
        outline,
    );
    let c = xy(-0.02, -0.36);
    let r = s * 0.12;
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
            .map(|(x, y)| xy(x, y - 0.1))
            .to_vec();
            p.add(egui::Shape::line(q, Stroke::new((s * 0.045).max(1.6), INK)));
            p.circle_filled(xy(0.03, -0.18), (s * 0.025).max(1.), INK);
        }
        Some(Terrain::Forest) => {
            p.line_segment([c, c + egui::vec2(0., r)], Stroke::new(1.5, INK));
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

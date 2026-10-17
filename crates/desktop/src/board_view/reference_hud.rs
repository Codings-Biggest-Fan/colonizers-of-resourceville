use super::*;

pub(super) const SEA: Color32 = Color32::from_rgb(7, 103, 166);
const DARK: Color32 = Color32::from_rgb(47, 47, 47);
const PAPER: Color32 = Color32::from_rgb(251, 248, 241);
const SAND: Color32 = Color32::from_rgb(227, 209, 174);

/// Measurements from the user's 2000 × 1135 reference. Only positions and
/// proportions live here; displayed inventories still come from the rules.
#[derive(Clone, Copy)]
struct Layout {
    screen: Rect,
    unit: f32,
}
impl Layout {
    fn new(screen: Rect) -> Self {
        Self {
            screen,
            unit: screen.width() / 2000.,
        }
    }
    fn right_width(self) -> f32 {
        self.unit * 451.
    }
    fn dock_height(self) -> f32 {
        self.unit * 110.
    }
    fn opponent_height(self) -> f32 {
        self.unit * 107.
    }
    fn active_height(self) -> f32 {
        self.unit * 168.
    }
    fn bank_height(self) -> f32 {
        self.unit * 109.
    }
    fn board_right(self) -> f32 {
        self.screen.right() - self.right_width()
    }
}

fn surface(p: &egui::Painter, rect: Rect, top: Color32, bottom: Color32, border: Color32) {
    paint_surface(p, rect, top, bottom, border, 3., 1.);
}

pub(super) fn paint_surface(
    p: &egui::Painter,
    rect: Rect,
    top: Color32,
    bottom: Color32,
    border: Color32,
    radius: f32,
    width: f32,
) {
    let radius = radius.min(rect.width() * 0.5).min(rect.height() * 0.5);
    let centers = [
        rect.right_top() + egui::vec2(-radius, radius),
        rect.right_bottom() - egui::vec2(radius, radius),
        rect.left_bottom() + egui::vec2(radius, -radius),
        rect.left_top() + egui::vec2(radius, radius),
    ];
    let mut mesh = egui::Mesh::default();
    let color = |y: f32| {
        let t = ((y - rect.top()) / rect.height()).clamp(0., 1.);
        Color32::from_rgb(
            (top.r() as f32 * (1. - t) + bottom.r() as f32 * t) as u8,
            (top.g() as f32 * (1. - t) + bottom.g() as f32 * t) as u8,
            (top.b() as f32 * (1. - t) + bottom.b() as f32 * t) as u8,
        )
    };
    mesh.colored_vertex(rect.center(), color(rect.center().y));
    for (corner, center) in centers.into_iter().enumerate() {
        for step in 0..=8 {
            let a = (-90. + corner as f32 * 90. + step as f32 * 90. / 8.).to_radians();
            let pos = center + egui::vec2(a.cos(), a.sin()) * radius;
            mesh.colored_vertex(pos, color(pos.y));
        }
    }
    let count = mesh.vertices.len() as u32 - 1;
    for i in 0..count {
        mesh.add_triangle(0, 1 + i, 1 + (i + 1) % count);
    }
    p.add(egui::Shape::mesh(mesh));
    p.rect_stroke(
        rect,
        radius,
        Stroke::new(width, border),
        egui::StrokeKind::Inside,
    );
}

fn text(
    p: &egui::Painter,
    at: Pos2,
    align: Align2,
    value: impl ToString,
    size: f32,
    color: Color32,
) {
    p.text(
        at,
        align,
        value.to_string(),
        FontId::proportional(size),
        color,
    );
}

fn hidden_card(p: &egui::Painter, rect: Rect, development: bool) {
    let top = if development {
        Color32::from_rgb(178, 153, 226)
    } else {
        Color32::from_rgb(56, 179, 219)
    };
    let bottom = if development {
        Color32::from_rgb(102, 62, 174)
    } else {
        Color32::from_rgb(14, 73, 145)
    };
    surface(p, rect, top, bottom, Color32::from_rgb(66, 75, 91));
    p.rect_stroke(
        rect.shrink(rect.width() * 0.06),
        1.,
        Stroke::new(1., Color32::from_rgb(204, 233, 230)),
        egui::StrokeKind::Inside,
    );
    if development {
        let c = rect.center();
        let s = rect.width();
        p.circle_filled(c, s * 0.31, Color32::from_rgb(108, 175, 181));
        p.circle_stroke(
            c,
            s * 0.31,
            Stroke::new(1., Color32::from_rgb(228, 215, 123)),
        );
        p.line_segment(
            [
                c + egui::vec2(-s * 0.16, s * 0.21),
                c + egui::vec2(s * 0.17, -s * 0.20),
            ],
            Stroke::new(s * 0.09, CREAM),
        );
        p.add(egui::Shape::line(
            vec![
                c + egui::vec2(-s * 0.04, -s * 0.22),
                c + egui::vec2(s * 0.15, -s * 0.25),
                c + egui::vec2(s * 0.26, -s * 0.09),
            ],
            Stroke::new(s * 0.12, Color32::from_rgb(137, 168, 176)),
        ));
    } else {
        text(
            p,
            rect.center(),
            Align2::CENTER_CENTER,
            "?",
            rect.height() * 0.7,
            Color32::from_rgb(166, 229, 244),
        );
    }
}

fn bank_icon(p: &egui::Painter, r: Rect) {
    let c = r.center();
    let s = r.width() / 2.;
    let pt = |x: f32, y: f32| c + egui::vec2(x, y) * s;
    let outline = Color32::from_rgb(135, 99, 52);
    let fill = Color32::from_rgb(230, 203, 151);
    p.add(egui::Shape::convex_polygon(
        vec![pt(-0.9, -0.6), pt(0., -1.1), pt(0.9, -0.6)],
        fill,
        Stroke::new(1., outline),
    ));
    for x in [-0.65, -0.22, 0.22, 0.65] {
        p.rect_filled(
            Rect::from_min_max(pt(x - 0.1, -0.5), pt(x + 0.1, 0.75)),
            1.,
            fill,
        );
        p.rect_stroke(
            Rect::from_min_max(pt(x - 0.1, -0.5), pt(x + 0.1, 0.75)),
            1.,
            Stroke::new(1., outline),
            egui::StrokeKind::Inside,
        );
    }
    for y in [0.78, 0.94] {
        p.line_segment([pt(-0.92, y), pt(0.92, y)], Stroke::new(s * 0.12, outline));
    }
}

impl Preview {
    pub(super) fn reference_controls(&mut self, ctx: &egui::Context) {
        if !self.test_open {
            return;
        }
        let layout = Layout::new(ctx.screen_rect());
        let k = layout.unit;
        let view = self.game.as_ref().map(Game::view);
        let phase = view.as_ref().map(|v| v.phase);
        let color = view
            .as_ref()
            .map(|v| pieces::player(v.active.0))
            .unwrap_or(ACCENT);
        let mut dock = Rect::NOTHING;
        egui::SidePanel::right("measured_sidebar")
            .resizable(false)
            .exact_width(layout.right_width())
            .frame(egui::Frame::new().fill(SEA).inner_margin(0))
            .show(ctx, |ui| {
                let area = ui.available_rect_before_wrap();
                let gap = 6. * k;
                let count = view.as_ref().map(|v| v.resource_counts.len()).unwrap_or(4);
                let roster = (count.saturating_sub(1) as f32) * (layout.opponent_height() + gap)
                    + layout.active_height();
                let bank_y = area.bottom() - roster - gap - layout.bank_height();
                let log_h = (bank_y - area.top() - 2. * gap) * 280. / 504.;
                let log = Rect::from_min_size(area.min, egui::vec2(area.width(), log_h));
                let chat = Rect::from_min_max(
                    egui::pos2(area.left(), log.bottom() + gap),
                    egui::pos2(area.right(), bank_y - gap),
                );
                surface(ui.painter(), log, PAPER, SAND, Color32::WHITE);
                surface(ui.painter(), chat, PAPER, SAND, Color32::WHITE);
                ui.scope_builder(egui::UiBuilder::new().max_rect(log.shrink(18. * k)), |ui| {
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .max_height(log.height() - 36. * k)
                        .show(ui, |ui| {
                            if self.activity.is_empty() {
                                ui.label(
                                    egui::RichText::new("Turn engine test")
                                        .size(16. * k)
                                        .color(DARK),
                                );
                            }
                            for entry in &self.activity {
                                ui.label(egui::RichText::new(entry).size(16. * k).color(DARK));
                            }
                        });
                });
                let input = Rect::from_min_max(
                    chat.left_bottom() + egui::vec2(12. * k, -42. * k),
                    chat.right_bottom() - egui::vec2(12. * k, 12. * k),
                );
                ui.put(
                    input,
                    egui::TextEdit::singleline(&mut String::new())
                        .hint_text("Say, Hello")
                        .font(FontId::proportional(16. * k))
                        .interactive(false),
                )
                .on_hover_text("Chat arrives with online rooms.");
                let bank = Rect::from_min_size(
                    egui::pos2(area.left(), bank_y),
                    egui::vec2(area.width(), layout.bank_height()),
                );
                surface(ui.painter(), bank, PAPER, SAND, Color32::WHITE);
                bank_icon(
                    ui.painter(),
                    Rect::from_min_size(
                        bank.min + egui::vec2(22. * k, 22. * k),
                        egui::vec2(64. * k, 66. * k),
                    ),
                );
                for (i, kind) in [0usize, 3, 1, 2, 4, 5].into_iter().enumerate() {
                    let r = Rect::from_min_size(
                        bank.min + egui::vec2((115. + 57. * i as f32) * k, 10. * k),
                        egui::vec2(42. * k, 60. * k),
                    );
                    if kind == 5 {
                        hidden_card(ui.painter(), r, true);
                    } else {
                        self.measured_resource(ui.painter(), r, kind);
                    }
                    let value = view
                        .as_ref()
                        .and_then(|v| if kind < 5 { Some(v.bank[kind]) } else { None })
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "—".into());
                    text(
                        ui.painter(),
                        egui::pos2(r.center().x, bank.bottom() - 9. * k),
                        Align2::CENTER_BOTTOM,
                        value,
                        20. * k,
                        Color32::BLACK,
                    );
                }
                if let Some(view) = &view {
                    let mut y = bank.bottom() + gap;
                    for seat in (0..view.resource_counts.len())
                        .filter(|&s| s != view.active.0 as usize)
                        .chain(std::iter::once(view.active.0 as usize))
                    {
                        let active = seat == view.active.0 as usize;
                        let height = if active {
                            layout.active_height()
                        } else {
                            layout.opponent_height()
                        };
                        let row = Rect::from_min_size(
                            egui::pos2(area.left(), y),
                            egui::vec2(area.width(), height),
                        );
                        surface(
                            ui.painter(),
                            row,
                            if active {
                                PAPER
                            } else {
                                Color32::from_rgb(176, 175, 172)
                            },
                            if active {
                                SAND
                            } else {
                                Color32::from_rgb(159, 148, 126)
                            },
                            if active {
                                Color32::WHITE
                            } else {
                                Color32::from_rgb(173, 190, 187)
                            },
                        );
                        self.measured_player(ui, row, seat, view, k, active);
                        y += height + gap;
                    }
                }
            });
        egui::TopBottomPanel::bottom("measured_dock")
            .resizable(false)
            .exact_height(layout.dock_height() + if self.error.is_some() { 28. * k } else { 0. })
            .frame(egui::Frame::new().fill(SEA).inner_margin(0))
            .show(ctx, |ui| {
                dock = ui.available_rect_before_wrap();
                let hand =
                    Rect::from_min_size(dock.min, egui::vec2(862. * k, layout.dock_height()));
                surface(ui.painter(), hand, PAPER, SAND, Color32::WHITE);
                self.measured_hand(ui, hand, k);
                let vertex = if let Some(Target::Vertex(id)) = self.selected {
                    Some(VertexId(id as u32))
                } else {
                    None
                };
                let edge = if let Some(Target::Edge(id)) = self.selected {
                    Some(EdgeId(id as u32))
                } else {
                    None
                };
                for (i, kind) in [4u8, 0, 1, 2, 3, 5].into_iter().enumerate() {
                    let enabled = match kind {
                        0 => {
                            edge.is_some()
                                && matches!(phase, Some(Phase::Build | Phase::SetupRoad { .. }))
                        }
                        1 => {
                            vertex.is_some()
                                && matches!(
                                    phase,
                                    Some(Phase::Build | Phase::SetupSettlement { .. })
                                )
                        }
                        2 => vertex.is_some() && phase == Some(Phase::Build),
                        5 => phase == Some(Phase::Build),
                        _ => false,
                    };
                    let label = match kind {
                        0 => "Build road: timber and clay",
                        1 => "Build settlement: timber, clay, wool and grain",
                        2 => "Upgrade city: two grain and three ore",
                        3 => "Development cards arrive in M05",
                        4 => "Trading arrives in M05",
                        _ => "End turn",
                    };
                    let r = Rect::from_min_size(
                        dock.min + egui::vec2((868. + 114. * i as f32) * k, 0.),
                        egui::vec2(106. * k, layout.dock_height() - 4. * k),
                    );
                    let clicked = ui
                        .scope_builder(egui::UiBuilder::new().max_rect(r), |ui| {
                            self.hud_button(ui, label, enabled, kind, color, r.width())
                                .clicked()
                        })
                        .inner;
                    if clicked {
                        match kind {
                            0 => self
                                .submit(Action::Road(edge.expect("enabled road")), Outcome::None),
                            1 => self.submit(
                                Action::Settlement(vertex.expect("enabled settlement")),
                                Outcome::None,
                            ),
                            2 => self
                                .submit(Action::City(vertex.expect("enabled city")), Outcome::None),
                            5 => self.submit(Action::EndTurn, Outcome::None),
                            _ => {}
                        }
                    }
                }
                if let Some(error) = &self.error {
                    text(
                        ui.painter(),
                        dock.left_bottom(),
                        Align2::LEFT_BOTTOM,
                        error,
                        16. * k,
                        Color32::from_rgb(185, 25, 28),
                    );
                }
            });
        // Overlay controls sit over empty sea, as in the reference. Panels still
        // reserve board space; no screenshot is painted behind working controls.
        egui::Area::new(egui::Id::new("measured_turn"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(
                layout.board_right() - 453. * k,
                dock.top() - 58. * k,
            ))
            .show(ctx, |ui| {
                let size = egui::vec2(336. * k, 52. * k);
                let enabled = matches!(phase, None | Some(Phase::AwaitRoll | Phase::Build));
                let (r, response) = ui.allocate_exact_size(
                    size,
                    if enabled {
                        egui::Sense::click()
                    } else {
                        egui::Sense::hover()
                    },
                );
                surface(ui.painter(), r, PAPER, SAND, Color32::WHITE);
                let title = match phase {
                    None => "Start turn test",
                    Some(Phase::AwaitRoll) => "Roll Dice",
                    Some(Phase::SetupSettlement { .. }) => "Place settlement",
                    Some(Phase::SetupRoad { .. }) => "Place road",
                    Some(Phase::Build) => "End Turn",
                    Some(Phase::AwaitRobber) => "Robber pending · M05",
                };
                text(
                    ui.painter(),
                    r.center(),
                    Align2::CENTER_CENTER,
                    title,
                    20. * k,
                    DARK,
                );
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, title)
                });
                if response.clicked() {
                    match phase {
                        None => self.start_test(),
                        Some(Phase::AwaitRoll) => {
                            self.submit(Action::Roll, Outcome::Dice(self.dice))
                        }
                        Some(Phase::Build) => self.submit(Action::EndTurn, Outcome::None),
                        _ => {}
                    }
                }
                let timer = Rect::from_min_size(
                    r.right_top() + egui::vec2(6. * k, 0.),
                    egui::vec2(108. * k, 52. * k),
                );
                surface(ui.painter(), timer, PAPER, SAND, Color32::WHITE);
                text(
                    ui.painter(),
                    timer.center(),
                    Align2::CENTER_CENTER,
                    "—:—",
                    20. * k,
                    Color32::BLACK,
                );
                ui.interact(
                    timer,
                    egui::Id::new("unavailable_turn_timer"),
                    egui::Sense::hover(),
                )
                .on_hover_text("Turn timers are not implemented.");
            });
        if let Some(dice) = view.as_ref().and_then(|v| v.last_dice) {
            egui::Area::new(egui::Id::new("measured_dice"))
                .order(egui::Order::Foreground)
                .fixed_pos(egui::pos2(
                    layout.board_right() - 251. * k,
                    dock.top() - 196. * k,
                ))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 22. * k;
                        for value in dice {
                            Self::measured_die(ui, value, 96. * k);
                        }
                    });
                });
        }
        egui::Area::new(egui::Id::new("measured_left_tools"))
            .order(egui::Order::Foreground)
            .fixed_pos(layout.screen.min + egui::vec2(5. * k, 6. * k))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 0.;
                if self
                    .hud_button(
                        ui,
                        "Return to board preview · F4 reopens turn controls",
                        true,
                        10,
                        ACCENT,
                        60. * k,
                    )
                    .clicked()
                {
                    self.test_open = false;
                    self.reveal_hand = false;
                }
                if self
                    .hud_button(ui, "Game guide", true, 11, ACCENT, 60. * k)
                    .clicked()
                {
                    self.guide_open = !self.guide_open;
                }
                self.hud_button(ui, "Audio is not implemented", false, 12, ACCENT, 60. * k);
                if self
                    .hud_button(ui, "Board options", true, 13, ACCENT, 60. * k)
                    .clicked()
                {
                    self.options_open = !self.options_open;
                }
                self.hud_button(
                    ui,
                    "Live support is unavailable",
                    false,
                    14,
                    ACCENT,
                    60. * k,
                );
            });
    }

    fn measured_resource(&self, p: &egui::Painter, rect: Rect, kind: usize) {
        if kind == 1 {
            p.rect_filled(rect, 0., Color32::from_rgb(162, 198, 47));
            p.image(
                self.wool_texture.as_ref().expect("loaded wool SVG").id(),
                rect,
                Rect::from_min_max(egui::pos2(0.2, 0.08), egui::pos2(0.8, 0.92)),
                Color32::WHITE,
            );
            p.rect_stroke(rect, 1., Stroke::new(1., DARK), egui::StrokeKind::Inside);
            return;
        }
        let col = (kind % 3) as f32;
        let row = (kind / 3) as f32;
        p.image(
            self.texture.as_ref().expect("loaded atlas").id(),
            rect,
            Rect::from_min_max(
                egui::pos2((col + 0.08) / 3., (row + 0.02) / 2.),
                egui::pos2((col + 0.92) / 3., (row + 0.86) / 2.),
            ),
            Color32::WHITE,
        );
        p.rect_stroke(rect, 1., Stroke::new(1., DARK), egui::StrokeKind::Inside);
    }

    fn measured_hand(&mut self, ui: &mut egui::Ui, r: Rect, k: f32) {
        let response = ui.interact(r, egui::Id::new("measured_hand"), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                true,
                "Reveal or cover active synthetic seat hand",
            )
        });
        if response.clicked() {
            self.reveal_hand = !self.reveal_hand;
            response.request_focus();
        }
        let Some(game) = &self.game else {
            return;
        };
        let view = game.view();
        if self.reveal_hand {
            if let Ok(hand) = game.hand(view.active) {
                let mut x = r.left() + 14. * k;
                for (kind, count) in hand.into_iter().enumerate().filter(|(_, count)| *count > 0) {
                    let card = Rect::from_min_size(
                        egui::pos2(x, r.top() + 10. * k),
                        egui::vec2(58. * k, 82. * k),
                    );
                    self.measured_resource(ui.painter(), card, kind);
                    text(
                        ui.painter(),
                        card.right_top(),
                        Align2::RIGHT_TOP,
                        count,
                        20. * k,
                        Color32::WHITE,
                    );
                    x += 64. * k;
                }
            }
        } else {
            for i in 0..view.resource_counts[view.active.0 as usize].min(16) {
                hidden_card(
                    ui.painter(),
                    Rect::from_min_size(
                        r.min + egui::vec2((14. + i as f32 * 32.) * k, 10. * k),
                        egui::vec2(58. * k, 82. * k),
                    ),
                    false,
                );
            }
        }
        response.on_hover_text("Click to reveal or cover this synthetic seat's hand. Focus loss and turn change cover it.");
    }

    fn measured_player(
        &self,
        ui: &mut egui::Ui,
        r: Rect,
        seat: usize,
        view: &resourceville_rules::TurnView,
        k: f32,
        active: bool,
    ) {
        let p = ui.painter();
        let owner = PlayerId(seat as u8);
        let color = pieces::player(seat as u8);
        text(
            p,
            r.min + egui::vec2(if active { 225. } else { 54. } * k, 16. * k),
            if active {
                Align2::CENTER_TOP
            } else {
                Align2::LEFT_TOP
            },
            Self::player_name(seat as u8),
            if active { 25. } else { 20. } * k,
            DARK,
        );
        let y = if active { 97. } else { 41. };
        let avatar = r.min + egui::vec2(57. * k, if active { 116. } else { 80. } * k);
        p.circle_filled(avatar, if active { 44. } else { 23. } * k, color);
        p.circle_stroke(
            avatar,
            if active { 44. } else { 23. } * k,
            Stroke::new(1., Color32::WHITE),
        );
        pieces::house(p, avatar, if active { 20. } else { 11. } * k, CREAM, false);
        for (i, development) in [false, true].into_iter().enumerate() {
            let card = Rect::from_min_size(
                r.min
                    + egui::vec2(
                        (115. + 57. * i as f32) * k,
                        if active { 65. } else { 10. } * k,
                    ),
                egui::vec2(42. * k, 60. * k),
            );
            hidden_card(p, card, development);
            text(
                p,
                card.center_bottom() + egui::vec2(0., 9. * k),
                Align2::CENTER_TOP,
                if development {
                    "—".into()
                } else {
                    view.resource_counts[seat].to_string()
                },
                20. * k,
                Color32::BLACK,
            );
        }
        // Awards and points are not part of M04. Reserve the reference slots
        // honestly, rather than inventing scores or knight/road award values.
        for (i, kind) in [0u8, 1, 2].into_iter().enumerate() {
            let at = r.min
                + egui::vec2(
                    (245. + 76. * i as f32) * k,
                    if active { 83. } else { 28. } * k,
                );
            if kind == 0 {
                p.circle_stroke(
                    at,
                    13. * k,
                    Stroke::new(2. * k, Color32::from_rgb(184, 169, 83)),
                );
            } else if kind == 1 {
                for x in [-7., 0., 7.] {
                    p.circle_stroke(
                        at + egui::vec2(x * k, 0.),
                        5. * k,
                        Stroke::new(1., Color32::GRAY),
                    );
                }
            } else {
                p.add(egui::Shape::line(
                    vec![
                        at + egui::vec2(-14., 5.) * k,
                        at + egui::vec2(-5., -5.) * k,
                        at + egui::vec2(5., -5.) * k,
                        at + egui::vec2(14., 5.) * k,
                    ],
                    Stroke::new(2. * k, Color32::GRAY),
                ));
            }
            text(
                p,
                at + egui::vec2(31. * k, 0.),
                Align2::CENTER_CENTER,
                "—",
                20. * k,
                Color32::BLACK,
            );
        }
        let roads = 15 - view.roads.values().filter(|&&p| p == owner).count();
        let settlements = 5 - view
            .buildings
            .values()
            .filter(|&&(p, kind)| p == owner && kind == Building::Settlement)
            .count();
        let cities = 4 - view
            .buildings
            .values()
            .filter(|&&(p, kind)| p == owner && kind == Building::City)
            .count();
        for (i, stock) in [roads, settlements, cities].into_iter().enumerate() {
            let at = r.min + egui::vec2((245. + 76. * i as f32) * k, (y + 40.) * k);
            if i == 0 {
                pieces::road(
                    p,
                    [at - egui::vec2(0., 13. * k), at + egui::vec2(0., 13. * k)],
                    90. * k,
                    color,
                );
            } else {
                pieces::house(p, at, 10. * k, color, i == 2);
            }
            text(
                p,
                at + egui::vec2(31. * k, 0.),
                Align2::CENTER_CENTER,
                stock,
                20. * k,
                Color32::BLACK,
            );
        }
        ui.interact(r,egui::Id::new(("measured_player",seat)),egui::Sense::hover()).on_hover_text("Public card total and remaining pieces. Scores, awards and development cards arrive in M05.");
    }

    fn measured_die(ui: &mut egui::Ui, value: u8, size: f32) {
        let (r, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
        let p = ui.painter();
        paint_surface(
            p,
            r,
            Color32::from_rgb(224, 248, 255),
            Color32::from_rgb(185, 232, 247),
            Color32::from_rgb(58, 91, 110),
            size * 0.12,
            size * 0.025,
        );
        let spots: &[(f32, f32)] = match value {
            1 => &[(0., 0.)],
            2 => &[(-1., -1.), (1., 1.)],
            3 => &[(-1., -1.), (0., 0.), (1., 1.)],
            4 => &[(-1., -1.), (-1., 1.), (1., -1.), (1., 1.)],
            5 => &[(-1., -1.), (-1., 1.), (0., 0.), (1., -1.), (1., 1.)],
            _ => &[
                (-1., -1.),
                (-1., 0.),
                (-1., 1.),
                (1., -1.),
                (1., 0.),
                (1., 1.),
            ],
        };
        for &(x, y) in spots {
            p.circle_filled(
                r.center() + egui::vec2(x, y) * size * 0.24,
                size * 0.10,
                Color32::from_rgb(117, 150, 170),
            );
        }
        response.on_hover_text(format!("Supplied test die: {value}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_layout_matches_reference_panel_and_dock() {
        let l = Layout::new(Rect::from_min_size(Pos2::ZERO, egui::vec2(2000., 1135.)));
        assert_eq!(l.right_width(), 451.);
        assert_eq!(l.dock_height(), 110.);
        assert_eq!(l.board_right(), 1549.);
        assert_eq!(
            3. * (l.opponent_height() + 6.) + l.active_height() + 6. + l.bank_height(),
            622.
        );
    }
}

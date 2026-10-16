use crate::pieces;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use resourceville_rules::{
    Action, Board, Building, EdgeId, Game, GameConfig, Outcome, Phase, PlayerId, Terrain, VertexId,
};

pub const BASE: Color32 = Color32::from_rgb(30, 30, 46);
const TEXT: Color32 = Color32::from_rgb(205, 214, 244);
const SUBTEXT: Color32 = Color32::from_rgb(186, 194, 222);
const ACCENT: Color32 = Color32::from_rgb(137, 180, 250);
const INK: Color32 = Color32::from_rgb(37, 33, 39);
const CREAM: Color32 = Color32::from_rgb(249, 237, 207);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    Tile(usize),
    Edge(usize),
    Vertex(usize),
    Port(usize),
}
#[derive(Clone, Copy)]
pub struct Camera {
    pub zoom: f32,
    pub pan: Vec2,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            zoom: 1.,
            pan: Vec2::ZERO,
        }
    }
}
impl Camera {
    pub fn scale(self, r: Rect) -> f32 {
        (r.width() / 11.8).min(r.height() / 11.8).max(1.) * self.zoom
    }
    pub fn project(self, r: Rect, p: Vec2) -> Pos2 {
        r.center() + self.pan + p * self.scale(r)
    }
    pub fn unproject(self, r: Rect, p: Pos2) -> Vec2 {
        (p - r.center() - self.pan) / self.scale(r)
    }
    fn zoom_at(&mut self, r: Rect, p: Pos2, f: f32) {
        let w = self.unproject(r, p);
        self.zoom = (self.zoom * f).clamp(0.7, 3.5);
        self.pan = p - r.center() - w * self.scale(r);
        self.bound(r);
    }
    fn bound(&mut self, r: Rect) {
        self.pan = self.pan.clamp(-r.size() * 0.7, r.size() * 0.7);
    }
}
fn lattice(p: [i16; 2]) -> Vec2 {
    egui::vec2(p[0] as f32 * 3f32.sqrt() / 2., p[1] as f32 / 2.)
}
fn center(p: [i16; 2]) -> Vec2 {
    lattice([2 * p[0] + p[1], 3 * p[1]])
}
fn segment_distance(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_sq()).clamp(0., 1.);
    p.distance(a + ab * t)
}
fn port_position(b: &Board, c: Camera, r: Rect, i: usize) -> Pos2 {
    let edge = &b.edges[b.ports[i].edge.0 as usize];
    let [a, z] = edge
        .vertices
        .map(|v| c.project(r, lattice(b.vertices[v.0 as usize].lattice)));
    let mid = a + (z - a) * 0.5;
    mid + (mid - c.project(r, Vec2::ZERO)).normalized() * c.scale(r) * 0.85
}
pub fn hit(b: &Board, c: Camera, r: Rect, p: Pos2) -> Option<Target> {
    if !r.contains(p) {
        return None;
    }
    for i in 0..b.ports.len() {
        if port_position(b, c, r, i).distance(p) <= (c.scale(r) * 0.4).max(10.) {
            return Some(Target::Port(i));
        }
    }
    for v in &b.vertices {
        if c.project(r, lattice(v.lattice)).distance(p) <= (c.scale(r) * 0.22).min(9.) {
            return Some(Target::Vertex(v.id.0 as usize));
        }
    }
    for e in &b.edges {
        let [a, z] = e
            .vertices
            .map(|v| c.project(r, lattice(b.vertices[v.0 as usize].lattice)));
        if segment_distance(p, a, z) <= (c.scale(r) * 0.15).min(6.) {
            return Some(Target::Edge(e.id.0 as usize));
        }
    }
    let w = c.unproject(r, p);
    b.tiles
        .iter()
        .find(|t| {
            let d = w - center(t.axial);
            let x = d.x.abs();
            let y = d.y.abs();
            x <= 3f32.sqrt() / 2. && y <= 1. && x / 3f32.sqrt() + y <= 1.
        })
        .map(|t| Target::Tile(t.id.0 as usize))
}
fn atlas_cell(t: Terrain) -> usize {
    match t {
        Terrain::Forest => 0,
        Terrain::Pasture => 1,
        Terrain::Fields => 2,
        Terrain::Hills => 3,
        Terrain::Mountains => 4,
        Terrain::Desert => 5,
    }
}
fn textured_hex(
    painter: &egui::Painter,
    texture: egui::TextureId,
    points: &[Pos2],
    c: Pos2,
    scale: f32,
    t: Terrain,
) {
    let cell = atlas_cell(t);
    let col = (cell % 3) as f32;
    let row = (cell / 3) as f32;
    // Inset sampling keeps linear filtering inside each atlas cell.
    let uv = |p: Pos2| {
        egui::pos2(
            (col + 0.5 + (p.x - c.x) / (scale * 3f32.sqrt()) * 0.996) / 3.,
            (row + 0.5 + (p.y - c.y) / (scale * 2.) * 0.996) / 2.,
        )
    };
    let mut mesh = egui::Mesh::with_texture(texture);
    mesh.vertices.push(egui::epaint::Vertex {
        pos: c,
        uv: uv(c),
        color: Color32::WHITE,
    });
    for &p in points {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: p,
            uv: uv(p),
            color: Color32::WHITE,
        });
    }
    for i in 0..6 {
        mesh.add_triangle(0, i + 1, (i + 1) % 6 + 1);
    }
    painter.add(egui::Shape::mesh(mesh));
}
pub struct Preview {
    texture: Option<egui::TextureHandle>,
    game: Option<Game>,
    test_open: bool,
    dice: [u8; 2],
    pub board: Board,
    pub camera: Camera,
    pub selected: Option<Target>,
    seed_text: String,
    error: Option<String>,
    samples: bool,
    #[cfg(feature = "capture")]
    capture_art: bool,
}
impl Default for Preview {
    fn default() -> Self {
        Self {
            texture: None,
            game: None,
            test_open: false,
            dice: [1, 1],
            board: Board::base(42),
            camera: Camera::default(),
            selected: None,
            seed_text: "42".into(),
            error: None,
            samples: true,
            #[cfg(feature = "capture")]
            capture_art: false,
        }
    }
}
impl Preview {
    #[cfg(feature = "capture")]
    pub fn capture_scenario(&mut self, scenario: &str) {
        if scenario == "contrast" {
            self.capture_art = true;
            return;
        }
        if scenario == "zoomed-out" {
            self.camera.zoom = 0.7;
        }
        if scenario == "layout" {
            return;
        }
        self.test_open = true;
        self.game = Some(
            Game::new(
                GameConfig {
                    board_seed: self.board.seed,
                    ..Default::default()
                },
                (0..4).map(PlayerId).collect(),
            )
            .expect("capture config"),
        );
        if scenario == "setup" {
            return;
        }
        while matches!(
            self.game.as_ref().unwrap().view().phase,
            Phase::SetupSettlement { .. } | Phase::SetupRoad { .. }
        ) {
            let g = self.game.as_ref().unwrap();
            let action = g.legal_actions(g.view().active)[0];
            self.submit(action, Outcome::None);
        }
        self.submit(Action::Roll, Outcome::Dice([3, 3]));
        if scenario == "rejection" {
            self.selected = Some(Target::Vertex(0));
            self.submit(Action::Settlement(VertexId(0)), Outcome::None);
        }
    }
    fn regenerate(&mut self) {
        match self.seed_text.trim().parse::<u64>() {
            Ok(seed) => {
                self.board = Board::base(seed);
                self.error = None;
                self.selected = None;
                self.game = None;
            }
            Err(_) => {
                self.error = Some("Enter a whole seed from 0 to 18446744073709551615.".into())
            }
        }
    }
    fn description(&self, t: Target) -> String {
        if let Some(game) = &self.game {
            let view = game.view();
            match t {
                Target::Vertex(i) => {
                    if let Some((owner, kind)) = view.buildings.get(&VertexId(i as u32)) {
                        return format!(
                            "Player {} · {:?} · Intersection {}",
                            owner.0 + 1,
                            kind,
                            i + 1
                        );
                    }
                }
                Target::Edge(i) => {
                    if let Some(owner) = view.roads.get(&EdgeId(i as u32)) {
                        return format!("Player {} · Road · Edge {}", owner.0 + 1, i + 1);
                    }
                }
                _ => (),
            }
        }
        match t {
            Target::Tile(i) => {
                let t = &self.board.tiles[i];
                format!(
                    "{}{}",
                    t.terrain.label(),
                    t.number
                        .map(|n| format!(" · {n}"))
                        .unwrap_or_else(|| " · No production".into())
                )
            }
            Target::Edge(i) => format!(
                "Edge {} · {}",
                i + 1,
                if self.board.edges[i].tiles.len() == 1 {
                    "Coast"
                } else {
                    "Inland"
                }
            ),
            Target::Vertex(i) => format!(
                "Intersection {} · {} tiles",
                i + 1,
                self.board.vertices[i].tiles.len()
            ),
            Target::Port(i) => self.board.ports[i]
                .resource
                .map(|r| format!("{} harbor · Trade 2:1", r.label()))
                .unwrap_or_else(|| "General harbor · Trade 3:1".into()),
        }
    }
    pub fn show(&mut self, ctx: &egui::Context) {
        if self.texture.is_none() {
            let im = image::load_from_memory(include_bytes!("../../../assets/art/terrain-v4.png"))
                .expect("embedded original atlas")
                .to_rgba8();
            self.texture = Some(ctx.load_texture(
                "terrain-v4",
                egui::ColorImage::from_rgba_unmultiplied(
                    [im.width() as usize, im.height() as usize],
                    im.as_raw(),
                ),
                egui::TextureOptions::LINEAR,
            ));
        }
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BASE).inner_margin(0.)).show(ctx,|ui|{
            let(rect,response)=ui.allocate_exact_size(ui.available_size(),egui::Sense::click_and_drag());
            response.widget_info(||egui::WidgetInfo::labeled(egui::WidgetType::Button,true,"Island board. Enter selects; Ctrl+Right cycles tiles, edges and intersections."));
            if response.clicked_by(egui::PointerButton::Primary){response.request_focus();self.selected=response.interact_pointer_pos().and_then(|p|hit(&self.board,self.camera,rect,p));}
            if response.dragged_by(egui::PointerButton::Secondary){self.camera.pan+=ctx.input(|i|i.pointer.delta());self.camera.bound(rect);}
            if response.hovered(){let d=ctx.input(|i|i.smooth_scroll_delta.y);if d!=0.{self.camera.zoom_at(rect,ctx.input(|i|i.pointer.hover_pos()).unwrap_or(rect.center()),(d*0.002).exp());}}
            if response.has_focus(){ctx.input(|i|{
                for(key,delta)in[(egui::Key::ArrowLeft,egui::vec2(25.,0.)),(egui::Key::ArrowRight,egui::vec2(-25.,0.)),(egui::Key::ArrowUp,egui::vec2(0.,25.)),(egui::Key::ArrowDown,egui::vec2(0.,-25.))]{if i.key_pressed(key)&&!i.modifiers.ctrl{self.camera.pan+=delta;}}
                if i.key_pressed(egui::Key::Home){self.camera=Camera::default();}
                if i.key_pressed(egui::Key::Plus)||i.key_pressed(egui::Key::Equals){self.camera.zoom_at(rect,rect.center(),1.2);}
                if i.key_pressed(egui::Key::Minus){self.camera.zoom_at(rect,rect.center(),1./1.2);}
                if i.key_pressed(egui::Key::Enter)||i.key_pressed(egui::Key::Space){self.selected=self.selected.or(Some(Target::Tile(0)));}
                if i.key_pressed(egui::Key::ArrowRight)&&i.modifiers.ctrl{let next=match self.selected{Some(Target::Tile(i))=>i+1,Some(Target::Edge(i))=>20+i,Some(Target::Vertex(i))=>92+i,Some(Target::Port(i))=>(146+i)%154,None=>0};self.selected=Some(if next<19{Target::Tile(next)}else if next<91{Target::Edge(next-19)}else if next<145{Target::Vertex(next-91)}else{Target::Port(next-145)});}
                if i.key_pressed(egui::Key::Escape){self.selected=None;}
            });self.camera.bound(rect);}
            if ctx.input(|i|i.key_pressed(egui::Key::F4)){self.test_open=true;}
            response.context_menu(|ui| {
                ui.set_max_width(270.);
                ui.label("Base island · Layout preview");
                if let Some(target)=self.selected {ui.label(self.description(target));}
                ui.separator();ui.label("Board seed");
                let edit=ui.add(egui::TextEdit::singleline(&mut self.seed_text).desired_width(180.).char_limit(20));
                if ui.button("Generate island").clicked() || (edit.lost_focus() && ui.input(|i|i.key_pressed(egui::Key::Enter))){self.regenerate();}
                if let Some(error)=&self.error{ui.colored_label(Color32::from_rgb(243,139,168),error);}
                if self.game.is_none() {ui.checkbox(&mut self.samples,"Show sample pieces");}
                if ui.button("Turn engine test").clicked(){self.test_open=true;ui.close();}
                if ui.button("Fit board").clicked(){self.camera=Camera::default();ui.close();}
                ui.separator();
                ui.label(egui::RichText::new("Wheel to zoom · Right-drag to pan").small().color(SUBTEXT));
                ui.label(egui::RichText::new("Complete offline games arrive in M06.").small().color(SUBTEXT));
            });
            let hover=response.hover_pos().and_then(|p|hit(&self.board,self.camera,rect,p));
            if let Some(t)=hover{response.clone().on_hover_text(self.description(t));}
            self.paint(ui,rect,hover);
            if response.has_focus(){ui.painter().rect_stroke(rect.shrink(2.),0.,Stroke::new(2.,ACCENT),egui::StrokeKind::Inside);}
        });
        self.turn_controls(ctx);
    }

    fn submit(&mut self, action: Action, outcome: Outcome) {
        if let Some(game) = &mut self.game {
            let actor = game.view().active;
            self.error = game
                .apply(actor, action, outcome)
                .err()
                .map(|e| e.to_string());
        }
    }
    fn turn_controls(&mut self, ctx: &egui::Context) {
        if !self.test_open {
            return;
        }
        let mut open = true;
        egui::Window::new("Turn engine test").open(&mut open).default_pos(egui::pos2(12.,12.)).default_width(250.).resizable(true).show(ctx,|ui| {
            ui.set_max_width(300.);
            egui::ScrollArea::vertical().max_height((ctx.screen_rect().height()-100.).max(150.)).show(ui,|ui|{
            if self.game.is_none() {
                ui.label("Test setup and ordinary building turns. Complete games arrive in M06.");
                if ui.button("Start four-seat test").clicked() {
                    let config=GameConfig {board_seed:self.board.seed,..Default::default()};
                    match Game::new(config,(0..4).map(PlayerId).collect()) {Ok(game)=>{self.game=Some(game);self.selected=None;self.error=None;},Err(e)=>self.error=Some(e.to_string())}
                }
            } else {
                let view=self.game.as_ref().expect("test game").view();
                ui.heading(format!("Player {}",view.active.0+1));
                ui.label(match view.phase {
                    Phase::SetupSettlement {..}=>"Setup: place a settlement",
                    Phase::SetupRoad {..}=>"Setup: place an adjoining road",
                    Phase::AwaitRoll=>"Supply a test dice outcome",
                    Phase::Build=>"Build or end the turn",
                    Phase::AwaitRobber=>"Seven: robber resolution awaits M05",
                });
                if let Some(d)=view.last_dice {ui.label(format!("Dice {} + {} = {}",d[0],d[1],d[0]+d[1]));}
                if view.phase==Phase::AwaitRoll {
                    ui.horizontal(|ui| {
                        for (i,die) in self.dice.iter_mut().enumerate() {ui.add(egui::DragValue::new(die).range(1..=6).prefix(format!("Die {}: ",i+1)));}
                    });
                    if ui.button("Apply dice").clicked() {self.submit(Action::Roll,Outcome::Dice(self.dice));}
                }
                if let Some(target)=self.selected {
                    ui.separator();ui.label(self.description(target));
                    match target {
                        Target::Vertex(i)=> {
                            if ui.button("Place settlement").clicked(){self.submit(Action::Settlement(VertexId(i as u32)),Outcome::None);}
                            if view.phase==Phase::Build && ui.button("Upgrade to city").clicked(){self.submit(Action::City(VertexId(i as u32)),Outcome::None);}
                        }
                        Target::Edge(i)=>{if ui.button("Place road").clicked(){self.submit(Action::Road(EdgeId(i as u32)),Outcome::None);}},
                        _=>(),
                    }
                }
                if view.phase==Phase::Build && ui.button("End turn").clicked() {self.submit(Action::EndTurn,Outcome::None);}
                ui.separator();
                ui.collapsing("Resource counts",|ui| {for (i,count) in view.resource_counts.iter().enumerate(){ui.label(format!("Player {}: {} resources",i+1,count));}});
                if ui.button("Reset test").clicked(){self.game=None;self.error=None;}
            }
            if let Some(error)=&self.error {ui.colored_label(Color32::from_rgb(243,139,168),error);}
            });
        });
        self.test_open = open;
    }
    fn paint(&self, ui: &egui::Ui, rect: Rect, hover: Option<Target>) {
        let painter = ui.painter_at(rect);
        let camera = self.camera;
        let scale = camera.scale(rect);
        let point =
            |v: VertexId| camera.project(rect, lattice(self.board.vertices[v.0 as usize].lattice));
        painter.rect_filled(rect, 0., Color32::from_rgb(126, 199, 223));
        // Follow shared coastal endpoints into one closed surf line, avoiding gaps at joins.
        let mut coastal: Vec<_> = self
            .board
            .edges
            .iter()
            .filter(|e| e.tiles.len() == 1)
            .collect();
        let first = coastal.remove(0);
        let start = first.vertices[0];
        let mut next = first.vertices[1];
        let mut coast = vec![point(start), point(next)];
        while next != start {
            let at = coastal
                .iter()
                .position(|e| e.vertices.contains(&next))
                .expect("closed island coast");
            let e = coastal.remove(at);
            next = if e.vertices[0] == next {
                e.vertices[1]
            } else {
                e.vertices[0]
            };
            if next != start {
                coast.push(point(next));
            }
        }
        for (width, color) in [
            (0.37, Color32::from_rgb(177, 226, 231)),
            (0.19, Color32::from_rgb(239, 216, 167)),
        ] {
            painter.add(egui::Shape::closed_line(
                coast.clone(),
                Stroke::new(scale * width, color),
            ));
        }
        for tile in &self.board.tiles {
            let points: Vec<_> = tile.vertices.iter().map(|&v| point(v)).collect();
            let c = camera.project(rect, center(tile.axial));
            textured_hex(
                &painter,
                self.texture.as_ref().expect("loaded atlas").id(),
                &points,
                c,
                scale,
                tile.terrain,
            );
            let selected = self.selected == Some(Target::Tile(tile.id.0 as usize));
            let active = hover == Some(Target::Tile(tile.id.0 as usize));
            let mut outline = points.clone();
            outline.push(points[0]);
            painter.add(egui::Shape::line(
                outline,
                Stroke::new(
                    if selected || active { 3. } else { 1.5 },
                    if selected {
                        TEXT
                    } else if active {
                        ACCENT
                    } else {
                        Color32::from_rgb(65, 62, 55)
                    },
                ),
            ));
            if let Some(n) = tile.number {
                painter.circle_filled(
                    c + egui::vec2(0., scale * 0.035),
                    scale * 0.29,
                    Color32::from_rgba_unmultiplied(27, 30, 39, 100),
                );
                painter.circle_filled(c, scale * 0.27, CREAM);
                painter.circle_stroke(
                    c,
                    scale * 0.27,
                    Stroke::new(1.5, Color32::from_rgb(116, 91, 64)),
                );
                let ink = if n == 6 || n == 8 {
                    Color32::from_rgb(146, 45, 36)
                } else {
                    INK
                };
                painter.text(
                    c - egui::vec2(0., scale * 0.025),
                    Align2::CENTER_CENTER,
                    n.to_string(),
                    FontId::proportional((scale * 0.34).clamp(10., 28.)),
                    ink,
                );
                let dots = 6 - (7i16 - n as i16).abs();
                for i in 0..dots {
                    painter.circle_filled(
                        c + egui::vec2(
                            (i as f32 - (dots - 1) as f32 / 2.) * scale * 0.067,
                            scale * 0.18,
                        ),
                        scale * 0.021,
                        ink,
                    );
                }
            }
        }
        for (i, port) in self.board.ports.iter().enumerate() {
            let edge = &self.board.edges[port.edge.0 as usize];
            let [a, b] = edge.vertices.map(point);
            let label = port_position(&self.board, camera, rect, i);
            let mid = a + (b - a) * 0.5;
            painter.line_segment(
                [mid, label],
                Stroke::new((scale * 0.14).max(3.), Color32::from_rgb(97, 65, 44)),
            );
            painter.line_segment(
                [mid, label],
                Stroke::new((scale * 0.08).max(2.), Color32::from_rgb(210, 163, 103)),
            );
            pieces::ship(&painter, label, scale * 0.95, port.resource);
            if self.selected == Some(Target::Port(i)) || hover == Some(Target::Port(i)) {
                painter.circle_stroke(label, scale * 0.43, Stroke::new(2., TEXT));
            }
        }
        if let Some(game) = &self.game {
            let view = game.view();
            for (&edge, owner) in &view.roads {
                pieces::road(
                    &painter,
                    self.board.edges[edge.0 as usize].vertices.map(point),
                    scale,
                    pieces::player(owner.0),
                );
            }
            for (&v, &(owner, kind)) in &view.buildings {
                pieces::house(
                    &painter,
                    point(v),
                    (scale * 0.19).max(5.),
                    pieces::player(owner.0),
                    kind == Building::City,
                );
                pieces::seat_number(&painter, point(v) - egui::vec2(0., scale * 0.10), owner.0);
            }
            if self.test_open {
                for action in game.legal_actions(view.active) {
                    match action {
                        Action::Settlement(v) => {
                            painter.circle_stroke(point(v), 6., Stroke::new(2., INK));
                            painter.circle_stroke(point(v), 4., Stroke::new(2., CREAM));
                        }
                        Action::Road(e) => {
                            let [a, b] = self.board.edges[e.0 as usize].vertices.map(point);
                            let mid = a + (b - a) * 0.5;
                            painter.circle_filled(mid, 4., INK);
                            painter.circle_filled(mid, 2., CREAM);
                        }
                        _ => (),
                    }
                }
            }
        } else if self.samples {
            let [a, b] = self.board.edges[30].vertices.map(point);
            pieces::road(&painter, [a, b], scale, ACCENT);
            pieces::house(&painter, a, (scale * 0.19).max(5.), ACCENT, false);
        }
        #[cfg(feature = "capture")]
        if self.capture_art {
            for tile in &self.board.tiles {
                let ends = self.board.edges[tile.edges[0].0 as usize]
                    .vertices
                    .map(point);
                let color = pieces::player((tile.id.0 % 4) as u8);
                pieces::road(&painter, ends, scale, color);
                pieces::house(
                    &painter,
                    ends[0],
                    (scale * 0.19).max(5.),
                    color,
                    tile.id.0 % 2 == 0,
                );
            }
        }
        for (target, color) in [(self.selected, TEXT), (hover, ACCENT)] {
            match target {
                Some(Target::Vertex(i)) => {
                    painter.circle_stroke(
                        point(self.board.vertices[i].id),
                        10.,
                        Stroke::new(3., color),
                    );
                }
                Some(Target::Edge(i)) => {
                    painter.line_segment(
                        self.board.edges[i].vertices.map(point),
                        Stroke::new(4., color),
                    );
                }
                _ => {}
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_action_bridge_preserves_errors_and_reset_clears_game() {
        let mut p = Preview {
            test_open: true,
            game: Some(Game::new(GameConfig::default(), (0..4).map(PlayerId).collect()).unwrap()),
            ..Default::default()
        };
        let before = p.game.as_ref().unwrap().view();
        p.submit(Action::Road(EdgeId(0)), Outcome::None);
        assert!(p.error.is_some());
        assert_eq!(p.game.as_ref().unwrap().view(), before);
        let action = p.game.as_ref().unwrap().legal_actions(PlayerId(0))[0];
        p.submit(action, Outcome::None);
        assert!(p.error.is_none());
        assert!(matches!(
            p.game.as_ref().unwrap().view().phase,
            Phase::SetupRoad { .. }
        ));
        let ctx = egui::Context::default();
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(480., 420.))),
                ..Default::default()
            },
            |ctx| p.show(ctx),
        );
        p.seed_text = "99".into();
        p.regenerate();
        assert!(p.game.is_none());
        assert_eq!(p.board.seed, 99);
    }
    #[test]
    fn right_drag_pans_and_wheel_zooms_without_visible_controls() {
        let ctx = egui::Context::default();
        let mut preview = Preview::default();
        let mut frame = |events: Vec<egui::Event>| {
            let _ = ctx.run(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(1000., 680.))),
                    events,
                    ..Default::default()
                },
                |ctx| preview.show(ctx),
            );
            preview.camera
        };
        frame(vec![]);
        frame(vec![]);
        let start = egui::pos2(500., 340.);
        let end = start + egui::vec2(40., 25.);
        frame(vec![
            egui::Event::PointerMoved(start),
            egui::Event::PointerButton {
                pos: start,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: Default::default(),
            },
        ]);
        let camera = frame(vec![egui::Event::PointerMoved(end)]);
        assert_eq!(camera.pan, egui::vec2(40., 25.));
        frame(vec![egui::Event::PointerButton {
            pos: end,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: Default::default(),
        }]);
        let zoomed = frame(vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0., 100.),
            modifiers: Default::default(),
        }]);
        assert!(zoomed.zoom > camera.zoom);
        let zoomed_out = frame(vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0., -200.),
            modifiers: Default::default(),
        }]);
        assert!(zoomed_out.zoom < zoomed.zoom);
    }
    #[test]
    fn projection_and_hit_targets_survive_transforms() {
        let b = Board::base(42);
        for size in [
            egui::vec2(480., 220.),
            egui::vec2(1300., 720.),
            egui::vec2(700., 1000.),
        ] {
            let r = Rect::from_min_size(egui::pos2(16., 100.), size);
            for zoom in [0.7, 1., 2., 3.5] {
                for pan in [Vec2::ZERO, egui::vec2(13., -23.)] {
                    let c = Camera { zoom, pan };
                    for t in &b.tiles {
                        let w = center(t.axial);
                        let p = c.project(r, w);
                        assert!((c.unproject(r, p) - w).length() < 0.0001);
                        if r.contains(p) {
                            assert_eq!(hit(&b, c, r, p), Some(Target::Tile(t.id.0 as usize)));
                        }
                    }
                    for i in 0..b.ports.len() {
                        let p = port_position(&b, c, r, i);
                        if r.contains(p) {
                            assert_eq!(hit(&b, c, r, p), Some(Target::Port(i)));
                        }
                    }
                    for v in &b.vertices {
                        let p = c.project(r, lattice(v.lattice));
                        if r.contains(p) {
                            assert_eq!(hit(&b, c, r, p), Some(Target::Vertex(v.id.0 as usize)));
                        }
                    }
                    for e in &b.edges {
                        let [a, z] = e
                            .vertices
                            .map(|v| c.project(r, lattice(b.vertices[v.0 as usize].lattice)));
                        let p = a + (z - a) * 0.5;
                        if r.contains(p) {
                            assert_eq!(hit(&b, c, r, p), Some(Target::Edge(e.id.0 as usize)));
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn seed_errors_are_atomic_and_recoverable() {
        let mut p = Preview::default();
        let before = p.board.clone();
        for s in ["", "-1", "abc", "18446744073709551616"] {
            p.seed_text = s.into();
            p.regenerate();
            assert_eq!(p.board, before);
            assert!(p.error.is_some());
        }
        p.seed_text = "0".into();
        p.regenerate();
        assert_eq!(p.board.seed, 0);
        assert!(p.error.is_none());
    }
    #[test]
    fn cursor_anchored_zoom_and_limits() {
        let r = Rect::from_min_size(Pos2::ZERO, egui::vec2(1000., 700.));
        let p = egui::pos2(520., 330.);
        let mut c = Camera::default();
        let w = c.unproject(r, p);
        c.zoom_at(r, p, 1.2);
        assert!((c.unproject(r, p) - w).length() < 0.0001);
        c.zoom_at(r, p, 1e6);
        assert_eq!(c.zoom, 3.5);
        c.zoom_at(r, p, 1e-6);
        assert_eq!(c.zoom, 0.7);
    }
}

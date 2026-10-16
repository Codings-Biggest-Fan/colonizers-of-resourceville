//! Deterministic M04 transitions. Authoritative hands are never Debug/Serialize.
use crate::{
    Board, BoardLayout, CaravanVariants, DicePolicy, EdgeId, GameConfig, RobberPolicy, Ruleset,
    Scenario, SeatMode, Terrain, TurnOrder, VertexId,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const ENGINE_VERSION: u16 = 1;
pub type Resources = [u16; 5];
pub const ROAD_COST: Resources = [1, 0, 0, 1, 0];
pub const SETTLEMENT_COST: Resources = [1, 1, 1, 1, 0];
pub const CITY_COST: Resources = [0, 0, 2, 0, 3];
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerId(pub u8);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Building {
    Settlement,
    City,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    SetupSettlement {
        step: u8,
    },
    SetupRoad {
        step: u8,
        anchor: VertexId,
    },
    AwaitRoll,
    Build,
    /// M05 must resolve discards, movement and theft; never silently skip seven.
    AwaitRobber,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Settlement(VertexId),
    Road(EdgeId),
    City(VertexId),
    Roll,
    EndTurn,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    None,
    Dice([u8; 2]),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnError(pub &'static str);
impl std::fmt::Display for TurnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for TurnError {}
#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    config: GameConfig,
    board: Board,
    phase: Phase,
    active: PlayerId,
    order: Vec<PlayerId>,
    hands: Vec<Resources>,
    bank: Resources,
    roads: BTreeMap<EdgeId, PlayerId>,
    buildings: BTreeMap<VertexId, (PlayerId, Building)>,
    last_dice: Option<[u8; 2]>,
    revision: u64,
}
/// Public projection has no individual resource types, legal choices, or future outcomes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnView {
    pub phase: Phase,
    pub active: PlayerId,
    pub roads: BTreeMap<EdgeId, PlayerId>,
    pub buildings: BTreeMap<VertexId, (PlayerId, Building)>,
    pub resource_counts: Vec<u16>,
    pub bank: Resources,
    pub last_dice: Option<[u8; 2]>,
    pub revision: u64,
}
pub fn resource(terrain: Terrain) -> Option<usize> {
    match terrain {
        Terrain::Forest => Some(0),
        Terrain::Pasture => Some(1),
        Terrain::Fields => Some(2),
        Terrain::Hills => Some(3),
        Terrain::Mountains => Some(4),
        Terrain::Desert => None,
    }
}
impl Game {
    /// Seat permutation is supplied by the authority independently of the board seed.
    /// Config options beyond this milestone are explicitly rejected.
    pub fn new(config: GameConfig, order: Vec<PlayerId>) -> Result<Self, TurnError> {
        config
            .validate()
            .map_err(|_| TurnError("Invalid game configuration."))?;
        if config.ruleset != Ruleset::Base
            || config.scenario != Scenario::Heartland
            || !matches!(config.seat_mode, SeatMode::Standard | SeatMode::CasualTwo)
            || config.board != BoardLayout::Random
            || config.dice != DicePolicy::Independent
            || config.robber != RobberPolicy::Standard
            || config.discard_threshold != 7
            || config.turn_seconds.is_some()
            || config.victory_target.is_some()
            || config.caravans != CaravanVariants::default()
            || !config.player_trading
        {
            return Err(TurnError(
                "This turn engine supports only standard Heartland and casual two-seat setup; custom options arrive later.",
            ));
        }
        let expected: Vec<_> = (0..config.seats).collect();
        let mut actual: Vec<_> = order.iter().map(|p| p.0).collect();
        actual.sort_unstable();
        if actual != expected
            || (config.turn_order == TurnOrder::SeatOrder
                && order.iter().map(|p| p.0).collect::<Vec<_>>() != expected)
        {
            return Err(TurnError(
                "Supply each configured seat exactly once in the selected turn order.",
            ));
        }
        Ok(Self {
            board: Board::base(config.board_seed),
            phase: Phase::SetupSettlement { step: 0 },
            active: order[0],
            hands: vec![[0; 5]; config.seats as usize],
            bank: [19; 5],
            roads: BTreeMap::new(),
            buildings: BTreeMap::new(),
            last_dice: None,
            revision: 0,
            order,
            config,
        })
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn config(&self) -> &GameConfig {
        &self.config
    }
    pub fn view(&self) -> TurnView {
        TurnView {
            phase: self.phase,
            active: self.active,
            roads: self.roads.clone(),
            buildings: self.buildings.clone(),
            resource_counts: self.hands.iter().map(|h| h.iter().sum()).collect(),
            bank: self.bank,
            last_dice: self.last_dice,
            revision: self.revision,
        }
    }
    /// Seat must be resolved by the caller's authority, never by untrusted wire input.
    pub fn hand(&self, seat: PlayerId) -> Result<Resources, TurnError> {
        self.hands
            .get(seat.0 as usize)
            .copied()
            .ok_or(TurnError("Unknown seat."))
    }
    pub fn legal_actions(&self, actor: PlayerId) -> Vec<Action> {
        if actor != self.active {
            return vec![];
        }
        let mut candidates = Vec::new();
        match self.phase {
            Phase::SetupSettlement { .. } => {
                candidates.extend(self.board.vertices.iter().map(|v| Action::Settlement(v.id)))
            }
            Phase::SetupRoad { .. } => {
                candidates.extend(self.board.edges.iter().map(|e| Action::Road(e.id)))
            }
            Phase::AwaitRoll => candidates.push(Action::Roll),
            Phase::Build => {
                candidates.extend(
                    self.board
                        .vertices
                        .iter()
                        .flat_map(|v| [Action::Settlement(v.id), Action::City(v.id)]),
                );
                candidates.extend(self.board.edges.iter().map(|e| Action::Road(e.id)));
                candidates.push(Action::EndTurn);
            }
            Phase::AwaitRobber => (),
        }
        candidates
            .into_iter()
            .filter(|&a| self.validate(actor, a).is_ok())
            .collect()
    }
    fn count(&self, actor: PlayerId, kind: Building) -> usize {
        self.buildings
            .values()
            .filter(|&&(p, k)| p == actor && k == kind)
            .count()
    }
    fn can_pay(&self, actor: PlayerId, cost: Resources) -> bool {
        self.hands[actor.0 as usize]
            .iter()
            .zip(cost)
            .all(|(&n, c)| n >= c)
    }
    fn validate(&self, actor: PlayerId, action: Action) -> Result<(), TurnError> {
        let fail = |s| Err(TurnError(s));
        if actor != self.active {
            return fail("Wait for your turn.");
        }
        let setup = matches!(
            self.phase,
            Phase::SetupSettlement { .. } | Phase::SetupRoad { .. }
        );
        match (self.phase, action) {
            (Phase::SetupSettlement { .. } | Phase::Build, Action::Settlement(v)) => {
                let Some(vertex) = self.board.vertices.get(v.0 as usize) else {
                    return fail("Unknown intersection.");
                };
                if self.buildings.contains_key(&v)
                    || vertex.edges.iter().any(|e| {
                        self.board.edges[e.0 as usize]
                            .vertices
                            .iter()
                            .any(|u| self.buildings.contains_key(u))
                    })
                {
                    return fail("Keep one empty intersection between settlements.");
                }
                if self.count(actor, Building::Settlement) >= 5 {
                    return fail("All five settlements are in use.");
                }
                if !setup
                    && !vertex
                        .edges
                        .iter()
                        .any(|e| self.roads.get(e) == Some(&actor))
                {
                    return fail("Connect the settlement to your road.");
                }
                if !setup && !self.can_pay(actor, SETTLEMENT_COST) {
                    return fail("A settlement needs timber, wool, grain and clay.");
                }
            }
            (Phase::SetupRoad { anchor, .. }, Action::Road(_)) => {
                // Binding is handled below for both road phases.
                let _ = anchor;
            }
            (Phase::Build, Action::Road(_)) => (),
            (Phase::Build, Action::City(v)) => {
                if self.buildings.get(&v) != Some(&(actor, Building::Settlement)) {
                    return fail("Upgrade one of your settlements.");
                }
                if self.count(actor, Building::City) >= 4 {
                    return fail("All four cities are in use.");
                }
                if !self.can_pay(actor, CITY_COST) {
                    return fail("A city needs two grain and three ore.");
                }
            }
            (Phase::AwaitRoll, Action::Roll) | (Phase::Build, Action::EndTurn) => (),
            (Phase::AwaitRobber, _) => {
                return fail(
                    "Seven requires robber resolution, coming in M05. Reset the test to continue.",
                );
            }
            _ => return fail("That action is unavailable in this phase."),
        }
        if let Action::Road(e) = action {
            let Some(edge) = self.board.edges.get(e.0 as usize) else {
                return fail("Unknown road edge.");
            };
            if self.roads.contains_key(&e) {
                return fail("This edge already has a road.");
            }
            if self.roads.values().filter(|&&p| p == actor).count() >= 15 {
                return fail("All fifteen roads are in use.");
            }
            if let Phase::SetupRoad { anchor, .. } = self.phase {
                if !edge.vertices.contains(&anchor) {
                    return fail("Place the road next to the settlement just placed.");
                }
            } else {
                let connected = edge.vertices.iter().any(|v| match self.buildings.get(v) {
                    Some(&(owner, _)) => owner == actor,
                    None => self.board.vertices[v.0 as usize]
                        .edges
                        .iter()
                        .any(|e| self.roads.get(e) == Some(&actor)),
                });
                if !connected {
                    return fail(
                        "Connect the road to your network without crossing an opponent's settlement.",
                    );
                }
                if !self.can_pay(actor, ROAD_COST) {
                    return fail("A road needs timber and clay.");
                }
            }
        }
        Ok(())
    }
    /// All validation precedes mutation; clone-and-commit also protects future fallible steps.
    pub fn apply(
        &mut self,
        actor: PlayerId,
        action: Action,
        outcome: Outcome,
    ) -> Result<TurnView, TurnError> {
        self.validate(actor, action)?;
        match (action, outcome) {
            (Action::Roll, Outcome::Dice([a, b]))
                if (1..=6).contains(&a) && (1..=6).contains(&b) => {}
            (Action::Roll, _) => return Err(TurnError("Supply two dice between one and six.")),
            (_, Outcome::None) => (),
            _ => return Err(TurnError("Only a roll accepts a dice outcome.")),
        }
        let mut next = self.clone();
        next.transition(actor, action, outcome);
        next.revision += 1;
        *self = next;
        Ok(self.view())
    }
    fn pay(&mut self, actor: PlayerId, cost: Resources) {
        for (i, c) in cost.into_iter().enumerate() {
            self.hands[actor.0 as usize][i] -= c;
            self.bank[i] += c;
        }
    }
    fn transition(&mut self, actor: PlayerId, action: Action, outcome: Outcome) {
        match action {
            Action::Settlement(v) => {
                self.buildings.insert(v, (actor, Building::Settlement));
                if let Phase::SetupSettlement { step } = self.phase {
                    if step >= self.config.seats {
                        let mut demand = vec![[0; 5]; self.hands.len()];
                        for h in &self.board.vertices[v.0 as usize].tiles {
                            if let Some(r) = resource(self.board.tiles[h.0 as usize].terrain) {
                                demand[actor.0 as usize][r] += 1;
                            }
                        }
                        self.distribute(demand);
                    }
                    self.phase = Phase::SetupRoad { step, anchor: v };
                } else {
                    self.pay(actor, SETTLEMENT_COST);
                }
            }
            Action::Road(e) => {
                self.roads.insert(e, actor);
                if let Phase::SetupRoad { step, .. } = self.phase {
                    let next = step + 1;
                    let seats = self.config.seats;
                    if next == seats * 2 {
                        self.active = self.order[0];
                        self.phase = Phase::AwaitRoll;
                    } else {
                        let at = if next < seats {
                            next
                        } else {
                            seats * 2 - 1 - next
                        };
                        self.active = self.order[at as usize];
                        self.phase = Phase::SetupSettlement { step: next };
                    }
                } else {
                    self.pay(actor, ROAD_COST);
                }
            }
            Action::City(v) => {
                self.pay(actor, CITY_COST);
                self.buildings.insert(v, (actor, Building::City));
            }
            Action::Roll => {
                let Outcome::Dice(dice) = outcome else {
                    unreachable!()
                };
                self.last_dice = Some(dice);
                if dice[0] + dice[1] == 7 {
                    self.phase = Phase::AwaitRobber;
                    return;
                }
                let mut demand = vec![[0; 5]; self.hands.len()];
                for tile in &self.board.tiles {
                    if tile.number != Some(dice[0] + dice[1]) {
                        continue;
                    }
                    if let Some(r) = resource(tile.terrain) {
                        for v in tile.vertices {
                            if let Some(&(owner, kind)) = self.buildings.get(&v) {
                                demand[owner.0 as usize][r] +=
                                    if kind == Building::City { 2 } else { 1 };
                            }
                        }
                    }
                }
                self.distribute(demand);
                self.phase = Phase::Build;
            }
            Action::EndTurn => {
                let at = self
                    .order
                    .iter()
                    .position(|&p| p == actor)
                    .expect("validated active seat");
                self.active = self.order[(at + 1) % self.order.len()];
                self.phase = Phase::AwaitRoll;
                self.last_dice = None;
            }
        }
    }
    fn distribute(&mut self, demand: Vec<Resources>) {
        for r in 0..5 {
            let total: u16 = demand.iter().map(|d| d[r]).sum();
            let recipients = demand.iter().filter(|d| d[r] > 0).count();
            // A sole recipient takes what remains; multiple recipients get none on shortage.
            if total <= self.bank[r] || recipients == 1 {
                for (p, d) in demand.iter().enumerate() {
                    let n = d[r].min(self.bank[r]);
                    self.bank[r] -= n;
                    self.hands[p][r] += n;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn game(seats: u8) -> Game {
        Game::new(
            GameConfig {
                seats,
                seat_mode: if seats == 2 {
                    SeatMode::CasualTwo
                } else {
                    SeatMode::Standard
                },
                board_seed: 42,
                ..Default::default()
            },
            (0..seats).map(PlayerId).collect(),
        )
        .unwrap()
    }
    fn setup(g: &mut Game) {
        while matches!(
            g.phase,
            Phase::SetupSettlement { .. } | Phase::SetupRoad { .. }
        ) {
            let action = g.legal_actions(g.active)[0];
            g.apply(g.active, action, Outcome::None).unwrap();
        }
    }
    fn conserved(g: &Game) {
        for r in 0..5 {
            assert_eq!(g.bank[r] + g.hands.iter().map(|h| h[r]).sum::<u16>(), 19);
        }
    }
    fn grant(g: &mut Game, p: usize, amounts: Resources) {
        for (r, n) in amounts.into_iter().enumerate() {
            g.bank[r] -= n;
            g.hands[p][r] += n;
        }
    }
    #[test]
    fn snake_order_starting_resources_and_replay() {
        for seats in 2..=4 {
            for seed in 0..32 {
                let mut g = game(seats);
                g = Game::new(
                    GameConfig {
                        board_seed: seed,
                        ..g.config.clone()
                    },
                    g.order.clone(),
                )
                .unwrap();
                let mut replay = g.clone();
                let mut sequence = Vec::new();
                let mut expected = vec![[0; 5]; seats as usize];
                while let Phase::SetupSettlement { step } = g.phase {
                    sequence.push(g.active.0);
                    let action = g.legal_actions(g.active)[0];
                    if let Action::Settlement(v) = action
                        && step >= seats
                    {
                        for tile in &g.board.vertices[v.0 as usize].tiles {
                            if let Some(r) = resource(g.board.tiles[tile.0 as usize].terrain) {
                                expected[g.active.0 as usize][r] += 1;
                            }
                        }
                    }
                    for a in [Some(action), None] {
                        let a = a.unwrap_or_else(|| g.legal_actions(g.active)[0]);
                        let actor = g.active;
                        g.apply(actor, a, Outcome::None).unwrap();
                        replay.apply(actor, a, Outcome::None).unwrap();
                        assert!(g == replay);
                        conserved(&g);
                    }
                }
                let order: Vec<_> = (0..seats).chain((0..seats).rev()).collect();
                assert_eq!(sequence, order);
                assert_eq!(g.hands, expected);
                assert_eq!(g.phase, Phase::AwaitRoll);
                assert_eq!(g.roads.len(), seats as usize * 2);
                assert_eq!(g.buildings.len(), seats as usize * 2);
            }
        }
    }
    #[test]
    fn rejected_actions_are_atomic_and_outcomes_are_controlled() {
        let mut g = game(4);
        for (actor, a, o) in [
            (PlayerId(9), Action::Settlement(VertexId(0)), Outcome::None),
            (g.active, Action::Road(EdgeId(0)), Outcome::None),
            (
                g.active,
                Action::Settlement(VertexId(u32::MAX)),
                Outcome::None,
            ),
            (
                g.active,
                Action::Settlement(VertexId(0)),
                Outcome::Dice([3, 3]),
            ),
        ] {
            let before = g.clone();
            assert!(g.apply(actor, a, o).is_err());
            assert!(g == before);
        }
        let a = g.legal_actions(g.active)[0];
        g.apply(g.active, a, Outcome::None).unwrap();
        let anchor = match g.phase {
            Phase::SetupRoad { anchor, .. } => anchor,
            _ => unreachable!(),
        };
        let edge = g
            .board
            .edges
            .iter()
            .find(|e| !e.vertices.contains(&anchor))
            .unwrap()
            .id;
        let before = g.clone();
        assert!(
            g.apply(g.active, Action::Road(edge), Outcome::None)
                .is_err()
        );
        assert!(g == before);
        setup(&mut g);
        for dice in [[0, 1], [7, 2], [6, 0]] {
            let before = g.clone();
            assert!(
                g.apply(g.active, Action::Roll, Outcome::Dice(dice))
                    .is_err()
            );
            assert!(g == before);
        }
        g.apply(g.active, Action::Roll, Outcome::Dice([3, 4]))
            .unwrap();
        assert_eq!(g.phase, Phase::AwaitRobber);
        let before = g.clone();
        assert!(g.apply(g.active, Action::EndTurn, Outcome::None).is_err());
        assert!(g == before);
    }
    #[test]
    fn production_city_double_shortages_and_resource_conservation() {
        let mut g = game(4);
        let tile = g
            .board
            .tiles
            .iter()
            .find(|t| t.terrain == Terrain::Forest)
            .unwrap()
            .clone();
        g.buildings
            .insert(tile.vertices[0], (PlayerId(0), Building::City));
        g.buildings
            .insert(tile.vertices[2], (PlayerId(1), Building::Settlement));
        g.phase = Phase::AwaitRoll;
        let n = tile.number.unwrap();
        let dice = if n <= 7 { [1, n - 1] } else { [6, n - 6] };
        g.apply(g.active, Action::Roll, Outcome::Dice(dice))
            .unwrap();
        assert_eq!(g.hands[0][0], 2);
        assert_eq!(g.hands[1][0], 1);
        conserved(&g);
        // Two recipients: no payout for the short resource, other resources still paid.
        let mut short = game(4);
        grant(&mut short, 2, [18, 0, 0, 0, 0]);
        short.distribute(vec![[2, 1, 0, 0, 0], [1, 0, 0, 0, 0], [0; 5], [0; 5]]);
        assert_eq!(short.hands[0], [0, 1, 0, 0, 0]);
        assert_eq!(short.bank[0], 1);
        conserved(&short);
        short.distribute(vec![[2, 0, 0, 0, 0], [0; 5], [0; 5], [0; 5]]);
        assert_eq!(short.hands[0][0], 1);
        assert_eq!(short.bank[0], 0);
        conserved(&short);
    }
    #[test]
    fn costs_connectivity_distance_piece_limits_and_turn_rotation() {
        let mut g = game(3);
        setup(&mut g);
        let actor = g.active;
        assert!(g.apply(actor, Action::EndTurn, Outcome::None).is_err());
        g.apply(actor, Action::Roll, Outcome::Dice([1, 1])).unwrap();
        // Return setup resources for a controlled affordable construction fixture.
        for p in 0..g.hands.len() {
            for r in 0..5 {
                g.bank[r] += g.hands[p][r];
                g.hands[p][r] = 0;
            }
        }
        let city = g
            .buildings
            .iter()
            .find(|(_, (p, _))| *p == actor)
            .map(|(&v, _)| v)
            .unwrap();
        assert!(g.apply(actor, Action::City(city), Outcome::None).is_err());
        grant(&mut g, actor.0 as usize, CITY_COST);
        g.apply(actor, Action::City(city), Outcome::None).unwrap();
        assert_eq!(g.hands[actor.0 as usize], [0; 5]);
        assert_eq!(g.count(actor, Building::City), 1);
        conserved(&g);
        grant(&mut g, actor.0 as usize, [5, 5, 5, 5, 5]);
        let road = g
            .legal_actions(actor)
            .into_iter()
            .find(|a| matches!(a, Action::Road(_)))
            .unwrap();
        let before = g.hand(actor).unwrap();
        g.apply(actor, road, Outcome::None).unwrap();
        let after = g.hand(actor).unwrap();
        for r in 0..5 {
            assert_eq!(before[r] - after[r], ROAD_COST[r]);
        }
        conserved(&g);
        let disconnected = g
            .board
            .edges
            .iter()
            .find(|e| {
                !g.roads.contains_key(&e.id)
                    && e.vertices.iter().all(|v| {
                        !g.buildings.contains_key(v)
                            && g.board.vertices[v.0 as usize]
                                .edges
                                .iter()
                                .all(|e| g.roads.get(e) != Some(&actor))
                    })
            })
            .unwrap()
            .id;
        assert!(
            g.apply(actor, Action::Road(disconnected), Outcome::None)
                .is_err()
        );
        let adjacent = g.board.edges[g.board.vertices[city.0 as usize].edges[0].0 as usize]
            .vertices
            .into_iter()
            .find(|&v| v != city)
            .unwrap();
        assert!(
            g.apply(actor, Action::Settlement(adjacent), Outcome::None)
                .is_err()
        );
        let available: Vec<_> = g
            .board
            .edges
            .iter()
            .filter(|e| !g.roads.contains_key(&e.id))
            .map(|e| e.id)
            .collect();
        for e in available
            .into_iter()
            .take(15 - g.roads.values().filter(|&&p| p == actor).count())
        {
            g.roads.insert(e, actor);
        }
        assert!(
            !g.legal_actions(actor)
                .iter()
                .any(|a| matches!(a, Action::Road(_)))
        );
        g.apply(actor, Action::EndTurn, Outcome::None).unwrap();
        assert_eq!(g.active, PlayerId(1));
        assert_eq!(g.phase, Phase::AwaitRoll);
    }
    #[test]
    fn opponent_building_blocks_network_and_runtime_options_rejected() {
        let mut g = game(3);
        g.phase = Phase::Build;
        grant(&mut g, 0, ROAD_COST);
        let vertex = g
            .board
            .vertices
            .iter()
            .find(|v| v.edges.len() == 3)
            .unwrap()
            .clone();
        g.roads.insert(vertex.edges[0], PlayerId(0));
        g.buildings
            .insert(vertex.id, (PlayerId(1), Building::Settlement));
        assert!(
            g.apply(PlayerId(0), Action::Road(vertex.edges[1]), Outcome::None)
                .is_err()
        );
        for c in [
            GameConfig {
                dice: DicePolicy::BalancedBagV1,
                ..Default::default()
            },
            GameConfig {
                board: BoardLayout::Fixed,
                ..Default::default()
            },
            GameConfig {
                turn_seconds: Some(60),
                ..Default::default()
            },
        ] {
            assert!(Game::new(c, (0..4).map(PlayerId).collect()).is_err());
        }
        assert!(Game::new(GameConfig::default(), vec![PlayerId(0); 4]).is_err());
    }
    #[test]
    fn settlement_spending_and_reusable_piece_caps() {
        let mut g = game(4);
        g.phase = Phase::Build;
        let v = g.board.vertices[20].id;
        let e = g.board.vertices[20].edges[0];
        g.roads.insert(e, PlayerId(0));
        grant(&mut g, 0, SETTLEMENT_COST);
        g.apply(PlayerId(0), Action::Settlement(v), Outcome::None)
            .unwrap();
        assert_eq!(g.hands[0], [0; 5]);
        conserved(&g);
        // Controlled fixtures verify inventory caps independently of network shape.
        for id in [0, 4, 8, 12] {
            g.buildings
                .insert(VertexId(id), (PlayerId(0), Building::Settlement));
        }
        grant(&mut g, 0, SETTLEMENT_COST);
        assert!(
            !g.legal_actions(PlayerId(0))
                .iter()
                .any(|a| matches!(a, Action::Settlement(_)))
        );
        for id in [30, 34, 38, 42] {
            g.buildings
                .insert(VertexId(id), (PlayerId(0), Building::City));
        }
        grant(&mut g, 0, CITY_COST);
        let before = g.clone();
        assert!(
            g.apply(PlayerId(0), Action::City(v), Outcome::None)
                .is_err()
        );
        assert!(g == before);
        g.buildings.remove(&VertexId(42));
        g.apply(PlayerId(0), Action::City(v), Outcome::None)
            .unwrap();
        assert_eq!(g.count(PlayerId(0), Building::Settlement), 4);
        assert_eq!(g.count(PlayerId(0), Building::City), 4);
        conserved(&g);
    }
    #[test]
    fn ordinary_turn_replay_and_legal_actions_agree_with_validation() {
        let mut g = game(4);
        setup(&mut g);
        let mut replay = g.clone();
        for turn in 0..120 {
            let dice = match turn % 5 {
                0 => [3, 3],
                1 => [4, 4],
                2 => [2, 3],
                3 => [5, 4],
                _ => [1, 1],
            };
            let actor = g.active;
            g.apply(actor, Action::Roll, Outcome::Dice(dice)).unwrap();
            replay
                .apply(actor, Action::Roll, Outcome::Dice(dice))
                .unwrap();
            for _ in 0..3 {
                let legal = g.legal_actions(actor);
                let Some(action) = legal
                    .iter()
                    .find(|a| !matches!(a, Action::EndTurn))
                    .copied()
                else {
                    break;
                };
                g.apply(actor, action, Outcome::None).unwrap();
                replay.apply(actor, action, Outcome::None).unwrap();
                conserved(&g);
            }
            g.apply(actor, Action::EndTurn, Outcome::None).unwrap();
            replay.apply(actor, Action::EndTurn, Outcome::None).unwrap();
            assert!(g == replay);
            conserved(&g);
        }
    }
}

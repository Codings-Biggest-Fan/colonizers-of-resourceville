//! Generator v1: radius-two island; integer lattice identities never use pixels.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const BOARD_GENERATOR_VERSION: u16 = 1;
macro_rules! identity {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $name(pub u32);
    };
}
identity!(HexId);
identity!(VertexId);
identity!(EdgeId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Terrain {
    Forest,
    Pasture,
    Fields,
    Hills,
    Mountains,
    Desert,
}
impl Terrain {
    pub fn label(self) -> &'static str {
        match self {
            Self::Forest => "Forest",
            Self::Pasture => "Pasture",
            Self::Fields => "Fields",
            Self::Hills => "Hills",
            Self::Mountains => "Mountains",
            Self::Desert => "Desert",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Tile {
    pub id: HexId,
    pub axial: [i16; 2],
    pub vertices: [VertexId; 6],
    pub edges: [EdgeId; 6],
    pub terrain: Terrain,
    pub number: Option<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Vertex {
    pub id: VertexId,
    pub lattice: [i16; 2],
    pub tiles: Vec<HexId>,
    pub edges: Vec<EdgeId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Edge {
    pub id: EdgeId,
    pub vertices: [VertexId; 2],
    pub tiles: Vec<HexId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Port {
    pub edge: EdgeId,
    pub resource: Option<Terrain>,
}
/// Generated only through `base`: callers cannot deserialize unchecked topology.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Board {
    pub generator_version: u16,
    pub seed: u64,
    pub tiles: Vec<Tile>,
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub ports: Vec<Port>,
}

const CORNERS: [[i16; 2]; 6] = [[1, 1], [0, 2], [-1, 1], [-1, -1], [0, -2], [1, -1]];
impl Board {
    pub fn base(seed: u64) -> Self {
        let coords: Vec<[i16; 2]> = (-2i16..=2)
            .flat_map(|q| (-2i16..=2).filter_map(move |r| ((q + r).abs() <= 2).then_some([q, r])))
            .collect();
        let corner =
            |p: [i16; 2], i: usize| [2 * p[0] + p[1] + CORNERS[i][0], 3 * p[1] + CORNERS[i][1]];
        let points: BTreeSet<_> = coords
            .iter()
            .flat_map(|&p| (0..6).map(move |i| corner(p, i)))
            .collect();
        let vertex_ids: BTreeMap<_, _> = points
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p, VertexId(i as u32)))
            .collect();
        let pair = |a: VertexId, b: VertexId| if a < b { [a, b] } else { [b, a] };
        let pairs: BTreeSet<_> = coords
            .iter()
            .flat_map(|&p| {
                let vertex_ids = &vertex_ids;
                (0..6).map(move |i| {
                    pair(
                        vertex_ids[&corner(p, i)],
                        vertex_ids[&corner(p, (i + 1) % 6)],
                    )
                })
            })
            .collect();
        let edge_ids: BTreeMap<_, _> = pairs
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p, EdgeId(i as u32)))
            .collect();
        let mut board = Self {
            generator_version: BOARD_GENERATOR_VERSION,
            seed,
            tiles: Vec::new(),
            vertices: vertex_ids
                .iter()
                .map(|(&lattice, &id)| Vertex {
                    id,
                    lattice,
                    tiles: vec![],
                    edges: vec![],
                })
                .collect(),
            edges: edge_ids
                .iter()
                .map(|(&vertices, &id)| Edge {
                    id,
                    vertices,
                    tiles: vec![],
                })
                .collect(),
            ports: vec![],
        };
        let mut random = BoardRandom(seed);
        let mut terrain = vec![Terrain::Forest; 4];
        terrain.extend([Terrain::Pasture; 4]);
        terrain.extend([Terrain::Fields; 4]);
        terrain.extend([Terrain::Hills; 3]);
        terrain.extend([Terrain::Mountains; 3]);
        terrain.push(Terrain::Desert);
        random.shuffle(&mut terrain);
        for (i, &p) in coords.iter().enumerate() {
            let id = HexId(i as u32);
            let vertices = std::array::from_fn(|j| vertex_ids[&corner(p, j)]);
            let edges =
                std::array::from_fn(|j| edge_ids[&pair(vertices[j], vertices[(j + 1) % 6])]);
            for v in vertices {
                board.vertices[v.0 as usize].tiles.push(id);
            }
            for e in edges {
                board.edges[e.0 as usize].tiles.push(id);
            }
            board.tiles.push(Tile {
                id,
                axial: p,
                vertices,
                edges,
                terrain: terrain[i],
                number: None,
            });
        }
        for e in &board.edges {
            for v in e.vertices {
                board.vertices[v.0 as usize].edges.push(e.id);
            }
        }
        let mut eligible: Vec<usize> = (0..19).filter(|&i| terrain[i] != Terrain::Desert).collect();
        random.shuffle(&mut eligible);
        let mut red = Vec::new();
        // Finite search over at most C(18,4) choices; never retry random boards.
        fn choose(board: &Board, candidates: &[usize], red: &mut Vec<usize>) -> bool {
            if red.len() == 4 {
                return true;
            }
            for (at, &i) in candidates.iter().enumerate() {
                if red
                    .iter()
                    .all(|&j| !board.adjacent(HexId(i as u32), HexId(j as u32)))
                {
                    red.push(i);
                    if choose(board, &candidates[at + 1..], red) {
                        return true;
                    }
                    red.pop();
                }
            }
            false
        }
        assert!(
            choose(&board, &eligible, &mut red),
            "radius-two island has four independent producing tiles"
        );
        for (i, n) in red.iter().zip([6, 8, 6, 8]) {
            board.tiles[*i].number = Some(n);
        }
        let mut tokens = [2, 3, 3, 4, 4, 5, 5, 9, 9, 10, 10, 11, 11, 12];
        random.shuffle(&mut tokens);
        let mut numbers = tokens.into_iter();
        for i in eligible {
            if !red.contains(&i) {
                board.tiles[i].number = numbers.next();
            }
        }
        // Traverse the coast through shared vertices, independent of rendering.
        let coast: BTreeSet<EdgeId> = board
            .edges
            .iter()
            .filter(|e| e.tiles.len() == 1)
            .map(|e| e.id)
            .collect();
        let mut walk = Vec::new();
        let mut edge = *coast.first().expect("island coast");
        let mut vertex = board.edges[edge.0 as usize].vertices[0];
        for _ in 0..30 {
            walk.push(edge);
            let ends = board.edges[edge.0 as usize].vertices;
            vertex = if ends[0] == vertex { ends[1] } else { ends[0] };
            edge = *board.vertices[vertex.0 as usize]
                .edges
                .iter()
                .find(|&&id| id != edge && coast.contains(&id))
                .expect("closed coast");
        }
        let mut resources = [
            None,
            None,
            None,
            None,
            Some(Terrain::Forest),
            Some(Terrain::Pasture),
            Some(Terrain::Fields),
            Some(Terrain::Hills),
            Some(Terrain::Mountains),
        ];
        random.shuffle(&mut resources);
        board.ports = [0, 3, 7, 10, 13, 17, 20, 23, 27]
            .into_iter()
            .zip(resources)
            .map(|(i, resource)| Port {
                edge: walk[i],
                resource,
            })
            .collect();
        board
    }
    pub fn adjacent(&self, a: HexId, b: HexId) -> bool {
        a != b
            && self
                .tiles
                .get(a.0 as usize)
                .zip(self.tiles.get(b.0 as usize))
                .is_some_and(|(a, b)| a.edges.iter().any(|e| b.edges.contains(e)))
    }
}
/// Explicit wrapping SplitMix64; this stream is board-only, never game randomness.
struct BoardRandom(u64);
impl BoardRandom {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn shuffle<T>(&mut self, values: &mut [T]) {
        for i in (1..values.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            values.swap(i, j);
        }
    }
}

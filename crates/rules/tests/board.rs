use resourceville_rules::*;
use std::collections::BTreeSet;
#[test]
fn topology_is_shared_reciprocal_connected_and_seed_independent() {
    let board = Board::base(0);
    assert_eq!(
        (board.tiles.len(), board.vertices.len(), board.edges.len()),
        (19, 54, 72)
    );
    assert_eq!(
        board.edges.iter().filter(|e| e.tiles.len() == 1).count(),
        30
    );
    for tile in &board.tiles {
        for v in tile.vertices {
            assert!(board.vertices[v.0 as usize].tiles.contains(&tile.id));
        }
        for (i, e) in tile.edges.iter().enumerate() {
            let edge = &board.edges[e.0 as usize];
            assert!(edge.tiles.contains(&tile.id));
            assert!(edge.vertices.contains(&tile.vertices[i]));
            assert!(edge.vertices.contains(&tile.vertices[(i + 1) % 6]));
        }
        for other in &board.tiles {
            let shared_v = tile
                .vertices
                .iter()
                .filter(|v| other.vertices.contains(v))
                .count();
            let shared_e = tile
                .edges
                .iter()
                .filter(|e| other.edges.contains(e))
                .count();
            if tile.id == other.id {
                continue;
            }
            if board.adjacent(tile.id, other.id) {
                assert_eq!((shared_v, shared_e), (2, 1));
            } else {
                assert!(shared_v <= 1);
                assert_eq!(shared_e, 0);
            }
        }
    }
    for v in &board.vertices {
        assert!((1..=3).contains(&v.tiles.len()));
        assert!((2..=3).contains(&v.edges.len()));
        for e in &v.edges {
            assert!(board.edges[e.0 as usize].vertices.contains(&v.id));
        }
    }
    let mut reached = BTreeSet::from([VertexId(0)]);
    loop {
        let old = reached.len();
        for e in &board.edges {
            if e.vertices.iter().any(|v| reached.contains(v)) {
                reached.extend(e.vertices);
            }
        }
        if reached.len() == old {
            break;
        }
    }
    assert_eq!(reached.len(), 54);
    let other = Board::base(u64::MAX);
    assert_eq!(board.vertices, other.vertices);
    assert_eq!(board.edges, other.edges);
}
#[test]
fn seeded_inventories_numbers_and_ports() {
    for seed in (0..2048).chain([u64::MAX, u64::MAX - 1, 1 << 63]) {
        let b = Board::base(seed);
        for (terrain, count) in [
            (Terrain::Forest, 4),
            (Terrain::Pasture, 4),
            (Terrain::Fields, 4),
            (Terrain::Hills, 3),
            (Terrain::Mountains, 3),
            (Terrain::Desert, 1),
        ] {
            assert_eq!(
                b.tiles.iter().filter(|t| t.terrain == terrain).count(),
                count
            );
        }
        let mut numbers: Vec<_> = b.tiles.iter().filter_map(|t| t.number).collect();
        numbers.sort();
        assert_eq!(
            numbers,
            [2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12]
        );
        assert!(
            b.tiles
                .iter()
                .all(|t| (t.terrain == Terrain::Desert) == t.number.is_none())
        );
        for a in b.tiles.iter().filter(|t| matches!(t.number, Some(6 | 8))) {
            for c in b.tiles.iter().filter(|t| matches!(t.number, Some(6 | 8))) {
                assert!(!b.adjacent(a.id, c.id));
            }
        }
        assert_eq!(b.ports.len(), 9);
        let mut occupied = BTreeSet::new();
        for port in &b.ports {
            let edge = &b.edges[port.edge.0 as usize];
            assert_eq!(edge.tiles.len(), 1);
            for v in edge.vertices {
                assert!(occupied.insert(v));
            }
        }
        assert_eq!(b.ports.iter().filter(|p| p.resource.is_none()).count(), 4);
        for t in [
            Terrain::Forest,
            Terrain::Pasture,
            Terrain::Fields,
            Terrain::Hills,
            Terrain::Mountains,
        ] {
            assert_eq!(b.ports.iter().filter(|p| p.resource == Some(t)).count(), 1);
        }
    }
}
#[test]
fn generation_is_deterministic_and_wire_ids_unchanged() {
    assert_eq!(Board::base(42), Board::base(42));
    assert_ne!(Board::base(42).tiles, Board::base(43).tiles);
    assert_eq!(serde_json::to_string(&VertexId(17)).unwrap(), "17");
    let board = Board::base(42);
    let fixture = include_str!("fixtures/board-v1-seed42.json");
    assert_eq!(
        serde_json::to_value(&board).unwrap(),
        serde_json::from_str::<serde_json::Value>(fixture).unwrap()
    );
}

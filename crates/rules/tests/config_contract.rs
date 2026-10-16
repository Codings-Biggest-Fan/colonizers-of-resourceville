use resourceville_rules::*;

#[test]
fn pinned_config_fixture_and_combination_boundary() {
    let fixture: GameConfig = serde_json::from_str(include_str!("fixtures/base-v1.json")).unwrap();
    fixture.validate().unwrap();
    assert_eq!(
        fixture,
        GameConfig {
            board_seed: 42,
            ..Default::default()
        }
    );
    for scenario in [Scenario::NewHorizons, Scenario::DesertCrossing] {
        GameConfig {
            scenario,
            ruleset: Ruleset::SeasCitadels,
            ..Default::default()
        }
        .validate()
        .unwrap();
    }
    let mut value = serde_json::to_value(fixture).unwrap();
    value["hidden_deck_seed"] = 42.into();
    assert!(serde_json::from_value::<GameConfig>(value).is_err());
}

#[test]
fn default_and_all_seat_modes_round_trip() {
    for (seats, mode, ruleset, scenario) in [
        (4, SeatMode::Standard, Ruleset::Base, Scenario::Heartland),
        (2, SeatMode::CasualTwo, Ruleset::Base, Scenario::Heartland),
        (
            2,
            SeatMode::OfficialTwo,
            Ruleset::Caravans,
            Scenario::Shoals,
        ),
        (
            2,
            SeatMode::VoyagerTwo,
            Ruleset::Voyagers,
            Scenario::GrandVoyage,
        ),
        (
            5,
            SeatMode::PairedFiveSix,
            Ruleset::Base,
            Scenario::Heartland,
        ),
        (
            8,
            SeatMode::CustomSevenEight,
            Ruleset::Base,
            Scenario::Heartland,
        ),
    ] {
        let config = GameConfig {
            seats,
            seat_mode: mode,
            ruleset,
            scenario,
            ..Default::default()
        };
        config.validate().unwrap();
        assert_eq!(
            config,
            serde_json::from_str(&serde_json::to_string(&config).unwrap()).unwrap()
        );
        assert!(Capabilities::m02().require_playable(&config).is_err());
    }
}

#[test]
fn invalid_combinations_and_bounds_have_field_reasons() {
    let base = GameConfig::default();
    let cases = [
        (
            GameConfig {
                seats: 9,
                ..base.clone()
            },
            "seats",
        ),
        (
            GameConfig {
                seats: 2,
                ..base.clone()
            },
            "seats",
        ),
        (
            GameConfig {
                scenario: Scenario::Mistbound,
                ..base.clone()
            },
            "scenario",
        ),
        (
            GameConfig {
                ruleset: Ruleset::SeasCitadels,
                scenario: Scenario::Mistbound,
                ..base.clone()
            },
            "scenario",
        ),
        (
            GameConfig {
                schema_version: 2,
                ..base.clone()
            },
            "version",
        ),
        (
            GameConfig {
                victory_target: Some(0),
                ..base.clone()
            },
            "victory_target",
        ),
        (
            GameConfig {
                discard_threshold: 255,
                ..base.clone()
            },
            "discard_threshold",
        ),
        (
            GameConfig {
                turn_seconds: Some(0),
                ..base.clone()
            },
            "turn_seconds",
        ),
        (
            GameConfig {
                caravans: CaravanVariants {
                    event_deck: true,
                    ..Default::default()
                },
                ..base
            },
            "caravans",
        ),
    ];
    for (config, field) in cases {
        let error = config.validate().unwrap_err();
        assert_eq!(error.field, field);
        assert!(!error.reason.is_empty());
    }
}

#[test]
fn edits_clear_readiness_and_freeze_is_immutable() {
    let mut lobby = ConfigLobby::new(GameConfig::default()).unwrap();
    lobby.set_ready(0, 0, true).unwrap();
    assert!(!lobby.replace(0, lobby.config().clone()).unwrap());
    assert!(lobby.readiness()[0]);
    let next = GameConfig {
        board_seed: 42,
        ..lobby.config().clone()
    };
    assert!(lobby.replace(0, next).unwrap());
    assert_eq!(lobby.readiness(), &[false; 4]);
    assert!(lobby.freeze(1).is_err());
    for seat in 0..4 {
        lobby.set_ready(1, seat, true).unwrap();
    }
    let frozen = lobby.freeze(1).unwrap();
    assert_eq!(frozen.config().board_seed, 42);
    assert_eq!(frozen.revision(), 1);
    let before = lobby.clone();
    assert!(lobby.replace(1, GameConfig::default()).is_err());
    assert!(lobby.set_ready(1, 0, false).is_err());
    assert!(lobby.freeze(1).is_err());
    assert_eq!(lobby, before);
}

#[test]
fn rejected_edits_are_atomic_including_stale_revision_and_bad_seats() {
    let mut lobby = ConfigLobby::new(GameConfig::default()).unwrap();
    lobby.set_ready(0, 0, true).unwrap();
    let before = lobby.clone();
    assert!(
        lobby
            .replace(
                0,
                GameConfig {
                    seats: 255,
                    ..Default::default()
                }
            )
            .is_err()
    );
    assert!(lobby.replace(99, GameConfig::default()).is_err());
    assert!(lobby.set_ready(99, 0, false).is_err());
    assert!(lobby.set_ready(0, 255, true).is_err());
    assert_eq!(lobby, before);
}

#[test]
fn custom_marker_and_variant_conflicts() {
    assert!(!GameConfig::default().is_custom());
    assert!(
        GameConfig {
            dice: DicePolicy::BalancedBagV1,
            ..Default::default()
        }
        .is_custom()
    );
    let c = GameConfig {
        ruleset: Ruleset::Caravans,
        scenario: Scenario::Shoals,
        dice: DicePolicy::BalancedBagV1,
        caravans: CaravanVariants {
            event_deck: true,
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(c.validate().unwrap_err().field, "dice");
}

use resourceville_protocol::*;

#[test]
fn event_ingress_rejects_unknown_version_and_invalid_configuration() {
    let mut event = EventEnvelope {
        protocol_version: 1,
        game_id: 1,
        revision: 0,
        caused_by: 1,
        event: LobbyEvent::ConfigAccepted(GameConfig::default()),
    };
    assert_eq!(
        EventEnvelope::decode(&serde_json::to_vec(&event).unwrap()).unwrap(),
        event
    );
    event.protocol_version = 2;
    assert!(EventEnvelope::decode(&serde_json::to_vec(&event).unwrap()).is_err());
    event.protocol_version = 1;
    event.event = LobbyEvent::ConfigAccepted(GameConfig {
        seats: 0,
        ..Default::default()
    });
    assert!(EventEnvelope::decode(&serde_json::to_vec(&event).unwrap()).is_err());
}

#[test]
fn two_recipients_cannot_observe_each_others_inventory() {
    let config = GameConfig {
        seats: 2,
        seat_mode: resourceville_rules::SeatMode::CasualTwo,
        ..Default::default()
    };
    let state = ObservationState::new(
        PublicView {
            revision: 0,
            config,
            seats: vec![
                PublicSeat {
                    seat: SeatId(0),
                    resource_count: 5,
                    public_points: 2,
                },
                PublicSeat {
                    seat: SeatId(1),
                    resource_count: 10,
                    public_points: 2,
                },
            ],
        },
        vec![
            PrivateHand {
                seat: SeatId(0),
                resources: [1; 5],
                private_points: 1,
            },
            PrivateHand {
                seat: SeatId(1),
                resources: [2; 5],
                private_points: 4,
            },
        ],
    )
    .unwrap();
    for seat in 0..2 {
        let view = state.player(SeatId(seat)).unwrap();
        let data = serde_json::to_value(view).unwrap();
        assert_eq!(data["own"]["seat"], seat);
        assert_eq!(data["own"]["resources"][0], u64::from(seat + 1));
        assert!(
            data["public"]["seats"][usize::from(1 - seat)]
                .get("resources")
                .is_none()
        );
    }
    assert!(state.player(SeatId(2)).is_err());
    assert!(
        serde_json::to_value(state.spectator())
            .unwrap()
            .get("own")
            .is_none()
    );
}

#[test]
fn command_round_trip_and_reject_unknown_versions_fields_and_oversize() {
    let command = CommandEnvelope {
        protocol_version: 1,
        game_id: 7,
        command_id: 9,
        expected_revision: 0,
        command: LobbyCommand::ReplaceConfig(GameConfig::default()),
    };
    let mut bytes = serde_json::to_vec(&command).unwrap();
    assert_eq!(CommandEnvelope::decode(&bytes).unwrap(), command);
    let mut value = serde_json::to_value(&command).unwrap();
    value["protocol_version"] = 2.into();
    assert!(CommandEnvelope::decode(&serde_json::to_vec(&value).unwrap()).is_err());
    value["protocol_version"] = 1.into();
    value["actor"] = 3.into();
    assert!(CommandEnvelope::decode(&serde_json::to_vec(&value).unwrap()).is_err());
    bytes.resize(MAX_WIRE_BYTES + 1, b' ');
    assert!(CommandEnvelope::decode(&bytes).is_err());
    assert!(
        !CommandEnvelope::decode(br#"{"secret":"do-not-echo"}"#)
            .unwrap_err()
            .contains("do-not-echo")
    );
}

#[test]
fn save_versions_and_invalid_config_are_rejected() {
    let mut save = SaveEnvelope {
        save_version: 1,
        protocol_version: 1,
        config: GameConfig::default(),
        revision: 4,
        state: [1_u16, 2],
    };
    let bytes = serde_json::to_vec(&save).unwrap();
    assert_eq!(
        SaveEnvelope::<[u16; 2]>::decode(&bytes).unwrap().state,
        [1, 2]
    );
    save.save_version = 0;
    assert!(SaveEnvelope::<[u16; 2]>::decode(&serde_json::to_vec(&save).unwrap()).is_err());
    save.save_version = 1;
    save.config.seats = 255;
    assert!(SaveEnvelope::<[u16; 2]>::decode(&serde_json::to_vec(&save).unwrap()).is_err());
    assert!(SaveEnvelope::<[u16; 2]>::decode(&vec![b' '; MAX_SAVE_BYTES + 1]).is_err());
}

#[test]
fn spectator_serialization_contains_only_public_counts() {
    let view = PlayerView {
        public: PublicView {
            revision: 0,
            config: GameConfig::default(),
            seats: vec![PublicSeat {
                seat: SeatId(0),
                resource_count: 8,
                public_points: 2,
            }],
        },
        own: PrivateHand {
            seat: SeatId(0),
            resources: [2, 1, 3, 1, 1],
            private_points: 3,
        },
    };
    let public = serde_json::to_value(&view.public).unwrap();
    assert!(public.get("own").is_none());
    assert!(public["seats"][0].get("resources").is_none());
    assert!(public["seats"][0].get("private_points").is_none());
    assert_eq!(public["seats"][0]["resource_count"], 8);
    let encoded = serde_json::to_vec(&view).unwrap();
    let decoded: PlayerView = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(decoded.own.resources, [2, 1, 3, 1, 1]);
}

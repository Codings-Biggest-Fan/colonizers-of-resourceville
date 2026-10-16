//! Versioned contracts only; no gameplay transport or save engine is implemented.
#![forbid(unsafe_code)]

pub use resourceville_rules::{Capabilities, GameConfig};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub const PROTOCOL_VERSION: u16 = 1;
pub const SAVE_VERSION: u16 = 1;
pub const MAX_WIRE_BYTES: usize = 64 * 1024;
pub const MAX_SAVE_BYTES: usize = 1024 * 1024;

/// Public product identity; contains no credentials or game state.
pub const PRODUCT_NAME: &str = "The Colonizers of Resourceville";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeatId(pub u8);
// Topology IDs are assigned by the board model at M03, never screen coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VertexId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexId(pub u32);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandEnvelope {
    pub protocol_version: u16,
    pub game_id: u64,
    pub command_id: u64,
    pub expected_revision: u64,
    pub command: LobbyCommand,
}
// Identity comes from the authenticated transport, never a caller-supplied actor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum LobbyCommand {
    ReplaceConfig(GameConfig),
    SetReady(bool),
    AcceptConfig,
}

impl CommandEnvelope {
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let value: Self = decode_bounded(bytes, MAX_WIRE_BYTES)?;
        if value.protocol_version != PROTOCOL_VERSION {
            return Err("Unsupported protocol version; update the client.".into());
        }
        if let LobbyCommand::ReplaceConfig(config) = &value.command {
            config.validate().map_err(|e| e.to_string())?;
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventEnvelope {
    pub protocol_version: u16,
    pub game_id: u64,
    pub revision: u64,
    pub caused_by: u64,
    pub event: LobbyEvent,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum LobbyEvent {
    ConfigAccepted(GameConfig),
    Readiness { seat: SeatId, ready: bool },
    Rejected { code: RejectCode },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectCode {
    StaleRevision,
    NotAuthorized,
    InvalidConfig,
    NotReady,
    NotAvailable,
}

impl EventEnvelope {
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let value: Self = decode_bounded(bytes, MAX_WIRE_BYTES)?;
        if value.protocol_version != PROTOCOL_VERSION {
            return Err("Unsupported event protocol version; update the client.".into());
        }
        match &value.event {
            LobbyEvent::ConfigAccepted(config) => config.validate().map_err(|e| e.to_string())?,
            LobbyEvent::Readiness { seat, .. } if seat.0 >= 8 => {
                return Err("Invalid seat identifier.".into());
            }
            _ => (),
        }
        Ok(value)
    }
}

/// Public data contains counts only. No secret state can be serialized here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicSeat {
    pub seat: SeatId,
    pub resource_count: u16,
    pub public_points: u16,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicView {
    pub revision: u64,
    pub config: GameConfig,
    pub seats: Vec<PublicSeat>,
}
/// Deliberately not Debug: private hands must not leak into ordinary diagnostics.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerView {
    pub public: PublicView,
    pub own: PrivateHand,
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivateHand {
    pub seat: SeatId,
    pub resources: [u16; 5],
    pub private_points: u16,
}

/// A projection boundary fixture for M02. Authoritative secrets intentionally
/// implement neither Serialize nor Debug. Later engines extend this boundary.
pub struct ObservationState {
    public: PublicView,
    hands: Vec<PrivateHand>,
}
impl ObservationState {
    pub fn new(public: PublicView, hands: Vec<PrivateHand>) -> Result<Self, String> {
        public.config.validate().map_err(|e| e.to_string())?;
        if public.seats.len() != usize::from(public.config.seats)
            || hands.len() != public.seats.len()
        {
            return Err(
                "One public seat and private hand are required per configured seat.".into(),
            );
        }
        for (i, seat) in public.seats.iter().enumerate() {
            if usize::from(seat.seat.0) != i
                || hands[i].seat != seat.seat
                || u32::from(seat.resource_count)
                    != hands[i]
                        .resources
                        .iter()
                        .map(|&v| u32::from(v))
                        .sum::<u32>()
            {
                return Err("Seat order or public hand count is inconsistent.".into());
            }
        }
        Ok(Self { public, hands })
    }
    pub fn spectator(&self) -> &PublicView {
        &self.public
    }
    /// Caller supplies a seat resolved from authenticated membership, not wire input.
    pub fn player(&self, seat: SeatId) -> Result<PlayerView, String> {
        let own = self
            .hands
            .get(usize::from(seat.0))
            .ok_or("Seat is not in this game.")?;
        Ok(PlayerView {
            public: self.public.clone(),
            own: own.clone(),
        })
    }
}

/// Header shared by later atomic save implementations. T is authoritative offline
/// state, never a spectator/player view. Do not send this envelope over the wire.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveEnvelope<T> {
    pub save_version: u16,
    pub protocol_version: u16,
    pub config: GameConfig,
    pub revision: u64,
    pub state: T,
}
impl<T: DeserializeOwned> SaveEnvelope<T> {
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let value: Self = decode_bounded(bytes, MAX_SAVE_BYTES)?;
        if value.save_version != SAVE_VERSION || value.protocol_version != PROTOCOL_VERSION {
            return Err("Unsupported save/protocol version; open with a compatible build. No migration is available.".into());
        }
        value.config.validate().map_err(|e| e.to_string())?;
        Ok(value)
    }
}

fn decode_bounded<T: DeserializeOwned>(bytes: &[u8], limit: usize) -> Result<T, String> {
    if bytes.len() > limit {
        return Err("Input exceeds the permitted size.".into());
    }
    // Do not propagate parser text that can echo secrets from malicious payloads.
    serde_json::from_slice(bytes).map_err(|_| "Invalid or unsupported data structure.".into())
}

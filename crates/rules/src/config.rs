use serde::{Deserialize, Serialize};

pub const CONFIG_VERSION: u16 = 1;
pub const RULES_REVISION: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ruleset {
    Base,
    Citadels,
    OpenSeas,
    SeasCitadels,
    Caravans,
    Voyagers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scenario {
    Heartland,
    CitadelHeartland,
    NewHorizons,
    FourIsles,
    Mistbound,
    DesertCrossing,
    LostKin,
    WovenCoasts,
    CorsairStrongholds,
    GreatWorks,
    FreeArchipelago,
    Shoals,
    Riverlands,
    CaravanTrail,
    RaiderSiege,
    FreightRoutes,
    FirstLandfall,
    CorsairCamps,
    FishingGrounds,
    SpiceRoutes,
    GrandVoyage,
}

impl Scenario {
    pub fn ruleset(self) -> Ruleset {
        match self {
            Self::Heartland => Ruleset::Base,
            Self::CitadelHeartland => Ruleset::Citadels,
            Self::NewHorizons
            | Self::FourIsles
            | Self::Mistbound
            | Self::DesertCrossing
            | Self::LostKin
            | Self::WovenCoasts
            | Self::CorsairStrongholds
            | Self::GreatWorks
            | Self::FreeArchipelago => Ruleset::OpenSeas,
            Self::Shoals
            | Self::Riverlands
            | Self::CaravanTrail
            | Self::RaiderSiege
            | Self::FreightRoutes => Ruleset::Caravans,
            _ => Ruleset::Voyagers,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeatMode {
    Standard,
    CasualTwo,
    OfficialTwo,
    VoyagerTwo,
    PairedFiveSix,
    CustomSevenEight,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DicePolicy {
    Independent,
    BalancedBagV1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RobberPolicy {
    Standard,
    Friendly,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoardLayout {
    Fixed,
    Random,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnOrder {
    Random,
    SeatOrder,
}

/// Authoritative gameplay options only. Room credentials, identity, appearance,
/// and secret random-generator state must never be added here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameConfig {
    pub schema_version: u16,
    pub rules_revision: u16,
    pub ruleset: Ruleset,
    pub scenario: Scenario,
    pub seats: u8,
    pub seat_mode: SeatMode,
    pub board: BoardLayout,
    pub board_seed: u64,
    /// None means use the scenario's rulebook target, not a universal target.
    pub victory_target: Option<u8>,
    pub dice: DicePolicy,
    pub robber: RobberPolicy,
    pub discard_threshold: u8,
    pub player_trading: bool,
    pub turn_seconds: Option<u16>,
    pub turn_order: TurnOrder,
    pub caravans: CaravanVariants,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaravanVariants {
    pub friendly_robber: bool,
    pub event_deck: bool,
    pub harbor_master: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            schema_version: CONFIG_VERSION,
            rules_revision: RULES_REVISION,
            ruleset: Ruleset::Base,
            scenario: Scenario::Heartland,
            seats: 4,
            seat_mode: SeatMode::Standard,
            board: BoardLayout::Random,
            board_seed: 0,
            victory_target: None,
            dice: DicePolicy::Independent,
            robber: RobberPolicy::Standard,
            discard_threshold: 7,
            player_trading: true,
            turn_seconds: None,
            turn_order: TurnOrder::Random,
            caravans: CaravanVariants::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigError {
    pub field: String,
    pub reason: String,
}
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.reason)
    }
}
impl std::error::Error for ConfigError {}
pub(crate) fn invalid(field: &str, reason: &str) -> ConfigError {
    ConfigError {
        field: field.into(),
        reason: reason.into(),
    }
}

impl GameConfig {
    /// Validates the M02 contract, independently of runtime feature availability.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.schema_version != CONFIG_VERSION || self.rules_revision != RULES_REVISION {
            return Err(invalid(
                "version",
                "Unsupported configuration or rules revision; use a compatible build.",
            ));
        }
        let matched = self.scenario.ruleset() == self.ruleset
            || (self.ruleset == Ruleset::SeasCitadels
                && matches!(
                    self.scenario,
                    Scenario::NewHorizons | Scenario::DesertCrossing
                ));
        if !matched {
            return Err(invalid(
                "scenario",
                "This scenario is not supported by the selected rules family.",
            ));
        }
        let valid_seats = match self.seat_mode {
            SeatMode::Standard => (3..=4).contains(&self.seats),
            SeatMode::CasualTwo => self.seats == 2 && self.ruleset == Ruleset::Base,
            SeatMode::OfficialTwo => self.seats == 2 && self.ruleset == Ruleset::Caravans,
            SeatMode::VoyagerTwo => self.seats == 2 && self.ruleset == Ruleset::Voyagers,
            // Larger expansion layouts require explicit scenario contracts at M14/M17.
            SeatMode::PairedFiveSix => {
                (5..=6).contains(&self.seats) && self.ruleset == Ruleset::Base
            }
            SeatMode::CustomSevenEight => {
                (7..=8).contains(&self.seats) && self.ruleset == Ruleset::Base
            }
        };
        if !valid_seats {
            return Err(invalid(
                "seats",
                "Use 3–4 standard seats, 2 casual base seats, 2 official caravan seats, or an explicit large base preset.",
            ));
        }
        if self.victory_target.is_some_and(|n| !(3..=30).contains(&n)) {
            return Err(invalid(
                "victory_target",
                "Choose 3–30 points or the scenario default.",
            ));
        }
        if !(5..=20).contains(&self.discard_threshold) {
            return Err(invalid(
                "discard_threshold",
                "Choose a base threshold of 5–20; city-wall modifiers apply separately.",
            ));
        }
        if self.turn_seconds.is_some_and(|s| !(15..=600).contains(&s)) {
            return Err(invalid(
                "turn_seconds",
                "Choose 15–600 seconds or untimed play.",
            ));
        }
        if self.caravans != CaravanVariants::default() && self.ruleset != Ruleset::Caravans {
            return Err(invalid(
                "caravans",
                "These variants require Caravans & Raiders.",
            ));
        }
        if self.caravans.friendly_robber && self.robber != RobberPolicy::Standard {
            return Err(invalid(
                "robber",
                "Use either the caravan variant or the custom friendly policy, not both.",
            ));
        }
        if self.caravans.event_deck && self.dice != DicePolicy::Independent {
            return Err(invalid(
                "dice",
                "The event deck replaces production dice; balanced dice cannot also apply.",
            ));
        }
        if self.ruleset == Ruleset::Voyagers
            && (self.robber != RobberPolicy::Standard
                || self.caravans != CaravanVariants::default())
        {
            return Err(invalid(
                "robber",
                "Voyagers uses its own corsair rules; base robber variants do not apply.",
            ));
        }
        Ok(())
    }

    /// Derived rather than caller-controlled, so nonstandard rules cannot hide.
    pub fn is_custom(&self) -> bool {
        self.seat_mode == SeatMode::CasualTwo
            || self.seat_mode == SeatMode::CustomSevenEight
            || self.victory_target.is_some()
            || self.dice != DicePolicy::Independent
            || self.robber != RobberPolicy::Standard
            || self.discard_threshold != 7
            || !self.player_trading
            || self.turn_seconds.is_some()
            || self.turn_order != TurnOrder::Random
    }
}

/// Do not derive this from the catalogued contracts: no game is playable at M02.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    pub protocol_version: u16,
    pub playable_rulesets: Vec<Ruleset>,
}
impl Capabilities {
    pub fn m02() -> Self {
        Self {
            protocol_version: 1,
            playable_rulesets: vec![],
        }
    }
    pub fn require_playable(&self, config: &GameConfig) -> Result<(), ConfigError> {
        config.validate()?;
        // Future support must be scenario/option-specific, not a family-name check.
        Err(invalid(
            "capability",
            "Gameplay is not available in this contract-only build.",
        ))
    }
}

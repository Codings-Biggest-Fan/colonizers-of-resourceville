use crate::{ConfigError, GameConfig, invalid};

/// Pure readiness/freeze model, not a room server or a game-start implementation.
/// Callers authorize membership/host access before invoking these methods.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigLobby {
    config: GameConfig,
    revision: u64,
    ready: Vec<bool>,
    frozen: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrozenConfig {
    config: GameConfig,
    revision: u64,
}
impl FrozenConfig {
    pub fn config(&self) -> &GameConfig {
        &self.config
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
}
impl ConfigLobby {
    pub fn new(config: GameConfig) -> Result<Self, ConfigError> {
        config.validate()?;
        Ok(Self {
            ready: vec![false; usize::from(config.seats)],
            config,
            revision: 0,
            frozen: false,
        })
    }
    pub fn config(&self) -> &GameConfig {
        &self.config
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn readiness(&self) -> &[bool] {
        &self.ready
    }
    fn writable(&self, expected: u64) -> Result<u64, ConfigError> {
        if self.frozen {
            return Err(invalid(
                "config",
                "Configuration is frozen after acceptance.",
            ));
        }
        if expected != self.revision {
            return Err(invalid(
                "revision",
                "Configuration changed; refresh and retry.",
            ));
        }
        self.revision
            .checked_add(1)
            .ok_or_else(|| invalid("revision", "Revision limit reached."))
    }
    pub fn replace(&mut self, expected: u64, config: GameConfig) -> Result<bool, ConfigError> {
        let next = self.writable(expected)?;
        config.validate()?;
        if config == self.config {
            return Ok(false);
        }
        self.ready = vec![false; usize::from(config.seats)];
        self.config = config;
        self.revision = next;
        Ok(true)
    }
    pub fn set_ready(&mut self, expected: u64, seat: u8, ready: bool) -> Result<(), ConfigError> {
        self.writable(expected)?;
        let value = self
            .ready
            .get_mut(usize::from(seat))
            .ok_or_else(|| invalid("seat", "Seat is not in this lobby."))?;
        *value = ready;
        Ok(())
    }
    pub fn freeze(&mut self, expected: u64) -> Result<FrozenConfig, ConfigError> {
        self.writable(expected)?;
        if !self.ready.iter().all(|r| *r) {
            return Err(invalid(
                "ready",
                "Every occupied seat must accept the current configuration.",
            ));
        }
        self.frozen = true;
        Ok(FrozenConfig {
            config: self.config.clone(),
            revision: self.revision,
        })
    }
}

use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::{
    error::{Error, Result},
    model::TxCategory,
};

#[derive(Deserialize, Serialize)]
pub struct Heuristics {
    pub cex: HashMap<String, Vec<String>>,
    pub bridge: HashMap<String, Vec<String>>,
}

impl Heuristics {
    pub fn new() -> Self {
        Self {
            cex: HashMap::new(),
            bridge: HashMap::new(),
        }
    }

    pub fn push_cex(&mut self, name: impl Into<String>, address: impl Into<String>) {
        self.cex
            .entry(name.into())
            .or_default()
            .push(address.into())
    }

    pub fn push_bridge(&mut self, name: impl Into<String>, address: impl Into<String>) {
        self.bridge
            .entry(name.into())
            .or_default()
            .push(address.into());
    }

    pub fn push_by_tx_category(&mut self, category: &TxCategory, address: impl Into<String>) {
        match category {
            TxCategory::Foreign => self.push_cex("unknown", address),
            TxCategory::Bridge => self.push_bridge("unknown", address),
            _ => {}
        }
    }

    pub fn load<P>(path: P) -> Result<Heuristics>
    where
        P: AsRef<Path>,
    {
        let file = read_to_string(path)?;
        let heuristics: Heuristics = from_str(&file)?;
        Ok(heuristics)
    }

    pub fn write<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let json = to_string_pretty(self)?;

        if let Some(parent) = path.parent() {
            create_dir_all(parent)
                .map_err(|err| Error::from_io(err, "❌ Failed to create parent directory"))?;
        }

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn is_known_cex(&self, addr: &str) -> bool {
        self.cex
            .values()
            .flatten()
            .any(|cex| cex.eq_ignore_ascii_case(addr))
    }

    pub fn is_known_bridge(&self, addr: &str) -> bool {
        self.bridge
            .values()
            .flatten()
            .any(|cex| cex.eq_ignore_ascii_case(addr))
    }

    pub fn is_known_domestic(&self, from: &str, to: &str) -> bool {
        self.is_known_cex(from) && self.is_known_cex(to)
    }
}

impl Default for Heuristics {
    fn default() -> Self {
        Heuristics::new()
    }
}

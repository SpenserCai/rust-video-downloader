//! Orchestrator - coordinates the download process
//!
//! This module will be implemented in Phase 3

use crate::error::Result;

pub struct Orchestrator;

impl Orchestrator {
    pub fn new(_config: crate::utils::config::Config, _cli: &crate::cli::Cli) -> Result<Self> {
        unimplemented!("Orchestrator will be implemented in Phase 3")
    }

    pub async fn run(&self, _cli: crate::cli::Cli) -> Result<()> {
        unimplemented!("Orchestrator will be implemented in Phase 3")
    }
}

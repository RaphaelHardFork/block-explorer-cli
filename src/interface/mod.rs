use crate::{utils::cli::prompt, Result};

// region:			--- Interface

pub struct Interface {
    pub mode: InterfaceMode,
}

impl Interface {
    pub fn prompt() -> Result<()> {
        let input = prompt("Search address, block number, block or transaction hash")?;
        Ok(())
    }
}

pub enum InterfaceMode {
    Address,
    Block,
    Transaction,
}

// endregion:		--- Interface

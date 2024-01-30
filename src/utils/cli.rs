use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{explorer::network::networks, Result};

// region:			--- Text prompt

pub fn prompt(text: &str) -> Result<String> {
    let theme = theme();
    let input = Input::with_theme(&theme);
    let res = input.with_prompt(text).interact_text()?;

    Ok(res)
}

pub fn network_select() -> Result<u64> {
    let theme = theme();
    let networks = networks();
    let chain_id = Select::with_theme(&theme).items(&networks).interact()?;

    if let Some(network) = networks.get(chain_id) {
        Ok(network.chain_id)
    } else {
        Err("No network selected".into())
    }
}

// endregion:		--- Text prompt

// region:			--- Theme

fn theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_style: Style::new().for_stderr().black(),
        prompt_suffix: style("?".to_string()).black().for_stderr(),
        ..Default::default()
    }
}

// endregion:		--- Theme

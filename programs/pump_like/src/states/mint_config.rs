use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct MintConfig {
    pub seed: String,
}

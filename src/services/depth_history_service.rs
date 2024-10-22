use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Meta {
    pub end_asset_depth: String,
    pub end_lp_units: String,
    pub end_member_count: String,
    pub end_rune_depth: String,
    pub end_synth_units: String,
    pub end_time: String,
    pub luvi_increase: String,
    pub price_shift_loss: String,
    pub start_asset_depth: String,
    pub start_lp_units: String,
    pub start_member_count: String,
    pub start_rune_depth: String,
    pub start_synth_units: String,
    pub start_time: String,
}

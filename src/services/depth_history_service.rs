use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Meta {
    // couldnt follow snake case since that how its in api response of midgard
    pub endAssetDepth: String,
    pub endLPUnits: String,
    pub endMemberCount: String,
    pub endRuneDepth: String,
    pub endSynthUnits: String,
    pub endTime: String,
    pub luviIncrease: String,
    pub priceShiftLoss: String,
    pub startAssetDepth: String,
    pub startLPUnits: String,
    pub startMemberCount: String,
    pub startRuneDepth: String,
    pub startSynthUnits: String,
    pub startTime: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Interval {
    // couldnt follow snake case since that how its in api response of midgard
    pub assetDepth: String,
    pub assetPrice: String,
    pub assetPriceUSD: String,
    pub endTime: String,
    pub liquidityUnits: String,
    pub luvi: String,
    pub membersCount: String,
    pub runeDepth: String,
    pub startTime: String,
    pub synthSupply: String,
    pub synthUnits: String,
    pub units: String,
}

// imitating the actual midgard api response style
#[derive(Debug,Deserialize,Serialize)]
pub struct ApiResponse{
    // if count > 1 we get vector of interval type objects
    pub intervals : Vec<Interval>,
    pub meta : Meta
}
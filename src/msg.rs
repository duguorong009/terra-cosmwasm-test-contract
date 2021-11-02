use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetTaxRate {},
    GetTaxCap { denom: String },
    GetCalcWithdrawAmount { uusd_amount: u128 },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TaxRateResponse {
    pub tax_rate: u128,
    pub denominator: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TaxCapResponse {
    pub tax_cap: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CalcPossibleWithdrawAmount {
    pub possible_withdraw_amount: u128,
    pub tax_amount: u128,
}

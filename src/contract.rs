#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, Fraction, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use terra_cosmwasm::TerraQuerier;

use crate::error::ContractError;
use crate::msg::{
    CalcPossibleWithdrawAmount, ExecuteMsg, InstantiateMsg, QueryMsg, TaxCapResponse,
    TaxRateResponse,
};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:terra-cosmwasm-test-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use std::ops::Add;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTaxCap { denom } => to_binary(&query_tax_cap(deps, denom)?),
        QueryMsg::GetTaxRate {} => to_binary(&query_tax_rate(deps)?),
        QueryMsg::GetCalcWithdrawAmount { uusd_amount } => {
            to_binary(&calc_withdraw_amount(deps, uusd_amount)?)
        }
    }
}

fn query_tax_cap(deps: Deps, denom: String) -> StdResult<TaxCapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let tax_cap = querier.query_tax_cap(denom)?;
    Ok(TaxCapResponse {
        tax_cap: tax_cap.cap.u128(),
    })
}

fn query_tax_rate(deps: Deps) -> StdResult<TaxRateResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let tax_rate = querier.query_tax_rate()?;
    Ok(TaxRateResponse {
        tax_rate: tax_rate.rate.numerator(),
        denominator: tax_rate.rate.denominator(),
    })
}

fn calc_withdraw_amount(deps: Deps, uusd_amount: u128) -> StdResult<CalcPossibleWithdrawAmount> {
    let uusd_amount = Uint128::new(uusd_amount);
    let querier = TerraQuerier::new(&deps.querier);
    let tax_cap = querier.query_tax_cap("uusd")?;
    let tax_rate = querier.query_tax_rate()?;
    let tax_amount = tax_cap.cap.min(uusd_amount.multiply_ratio(
        Uint128::from(1_000_000_000_000_000_000u128),
        Uint128::from(1_000_000_000_000_000_000u128).add(Uint128::from(tax_rate.rate.numerator())),
    ));
    let possible_withdraw_amount = uusd_amount - tax_amount;
    Ok(CalcPossibleWithdrawAmount {
        possible_withdraw_amount: possible_withdraw_amount.u128(),
        tax_amount: tax_amount.u128(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let info = mock_info("creator", &coins(1000, "earth"));

        let msg = InstantiateMsg {};

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetTaxRate {}).unwrap();
        let value: TaxRateResponse = from_binary(&res).unwrap();
        println!("{}", value.tax_rate);
    }
}

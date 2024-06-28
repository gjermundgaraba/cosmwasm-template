use cosmwasm_std::{Binary, Deps, entry_point, Env, StdResult, to_json_binary};
use cw_storage_plus::Bound;
use crate::msg::{MyQueryResponse, QueryMsg};
use crate::state::MESSAGES;

pub const DEFAULT_LIMIT: u64 = 30;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::MyQuery {start_after, limit} => to_json_binary(&my_query(deps, start_after, limit)?),
    }
}

pub fn my_query(deps: Deps, start_after: Option<u64>, limit: Option<u64>) -> StdResult<MyQueryResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    let start = start_after.map(Bound::exclusive);

    let messages: Vec<String> = MESSAGES
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit as usize)
        .collect::<Result<Vec<(u64, String)>, _>>()?
        .into_iter()
        .map(|(_, msg)| msg)
        .collect();

    Ok(MyQueryResponse{
        my_answers: messages,
    })
}
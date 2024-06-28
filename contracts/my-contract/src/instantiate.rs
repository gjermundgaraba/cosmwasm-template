use cosmwasm_std::{DepsMut, entry_point, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msg::InstantiateMsg;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};

    use crate::instantiate::instantiate;
    use crate::msg::InstantiateMsg;

    #[test]
    fn test_initialization() {
        let mut deps = mock_dependencies();
        let creator = deps. api. addr_make("creator");

        let info = message_info(&creator, &coins(10000, "ustake"));

        let msg = InstantiateMsg {};

        let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());
    }
}

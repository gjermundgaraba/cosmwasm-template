use cosmwasm_std::{
    DepsMut, entry_point, Env, MessageInfo, Response,
};

use crate::error::ContractError;
use crate::msg::ExecuteMsg;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MyMsg {} => my_message(),
    }
}

pub fn my_message() -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "my_message"))
}

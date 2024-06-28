use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    MyMsg {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MyQueryResponse)]
    MyQuery {
        limit: Option<u64>,
        start_after: Option<u64>,
    },
}

#[cw_serde]
pub struct MyQueryResponse {
    pub my_answers: Vec<String>,
}
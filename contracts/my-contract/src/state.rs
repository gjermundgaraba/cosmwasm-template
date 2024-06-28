use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_macro::index_list;
use cw_storage_plus::{IndexedMap, Item, Map, MultiIndex};

pub const MESSAGE_COUNT: Item<u64> = Item::new("message_count");
pub const MESSAGES: Map<u64, String> = Map::new("messages");
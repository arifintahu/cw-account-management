use cosmwasm_std::{StdResult, Api, Addr};

pub fn map_validate(api: &dyn Api, addresses: &[String]) -> StdResult<Vec<Addr>> {
    addresses.iter().map(|addr| api.addr_validate(addr)).collect()
}

pub fn validate_addr(api: &dyn Api, address: &String) -> StdResult<Addr> {
    api.addr_validate(address)
}
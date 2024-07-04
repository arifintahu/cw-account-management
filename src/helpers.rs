use cosmwasm_std::{StdResult, Api, Addr};

pub fn map_validate(api: &dyn Api, addresses: &[String]) -> StdResult<Vec<Addr>> {
    addresses.iter().map(|addr| api.addr_validate(addr)).collect()
}

pub fn validate_addr(api: &dyn Api, address: &String) -> StdResult<Addr> {
    api.addr_validate(address)
}

pub fn is_valid_threshold(threshold: u8, len_signers: usize) -> bool {
    threshold > 0 && threshold <= len_signers.try_into().unwrap()
}

pub fn is_sufficient_signers(threshold: u8, len_signers: usize) -> bool {
    threshold <= len_signers.try_into().unwrap()
}
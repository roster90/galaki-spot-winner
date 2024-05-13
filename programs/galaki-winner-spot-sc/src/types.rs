use anchor_lang::prelude::*;



#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AuthRole {
    Admin,
    Operator,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub enum DepositStatus {
    Waiting,
    Open,
    Close,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub struct PackageItem {
    pub id: u64,     //8
    pub price: u64,  //8
    pub reward: u64, //8
}

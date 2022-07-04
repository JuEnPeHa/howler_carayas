use crate::*;

pub trait SaleViews {
    fn get_saled_ids(&self) -> Vec<u16>;
    fn get_saled_tokens(&self) -> Vec<Token>;
    fn get_token_by_id(&self, id: u16) -> Token;
    fn get_account_and_current_block(&self) -> (AccountId, U64);
    fn get_current_block(&self) -> U64;
}

#[near_bindgen]
impl SaleViews for Contract {
    fn get_saled_ids(&self) -> Vec<u16> {
        self.minted.keys().into_iter().map(|k| k.clone()).collect()
    }

    fn get_saled_tokens(&self) -> Vec<Token> {
        self.minted.values().into_iter().map(|v| v).collect()
    }

    fn get_token_by_id(&self, id: u16) -> Token {
        self.minted.get(&id).unwrap_or_else(|| panic!("Token with id {:?} not found", id))
    }
    fn get_account_and_current_block(&self) -> (AccountId, U64) {
        let account: AccountId = env::signer_account_id();
        let block: U64 = U64(env::block_height());
        (account, block)
    }
    fn get_current_block(&self) -> U64 {
        U64(env::block_height())
    }
}
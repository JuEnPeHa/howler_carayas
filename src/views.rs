use crate::*;

pub trait SaleViews {
    fn get_saled_ids(&self) -> Vec<U64>;
}

#[near_bindgen]
impl SaleViews for Contract {
    fn get_saled_ids(&self) -> Vec<U64> {
        self.minted.keys().into_iter().map(|k| U64(k).clone()).collect()
    }
}
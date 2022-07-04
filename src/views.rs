use crate::*;

pub trait SaleViews {
    fn get_saled_ids(&self) -> Vec<U64>;
    fn set_sold_id(&mut self, id: U64);
}

#[near_bindgen]
impl SaleViews for Contract {
    fn get_saled_ids(&self) -> Vec<U64> {
        self.minted.keys().into_iter().map(|k| U64(k).clone()).collect()
    }

    fn set_sold_id(&mut self, id:U64) {
      
    }
}
use crate::Orderbook;

pub struct DepthLevel {
    price: u64,
    total_qty: u64,
}

pub struct DepthSnapshot {
    bids: Vec<DepthLevel>,
    asks: Vec<DepthLevel>,
}

impl DepthLevel{
    pub fn new()
}

impl DepthSnapshot {
    pub fn from_engine(book: Orderbook) -> DepthSnapshot {
        
    }
}

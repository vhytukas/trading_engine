use crate::order::Order;
use serde::*;
use std::collections::VecDeque;

#[derive(Serialize, Debug)]
pub struct PriceLevel {
    pub orders: VecDeque<Order>,
    pub total_qty: u64,
}

impl PriceLevel {
    pub fn new() -> PriceLevel {
        PriceLevel {
            orders: VecDeque::new(),
            total_qty: 0,
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.total_qty += order.qty;
        self.orders.push_back(order);
    }

    pub fn reduce_qty(&mut self, filled: u64) {
        self.total_qty = self.total_qty.saturating_sub(filled);
    }
}

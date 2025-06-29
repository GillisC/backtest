use crate::{OrderType, Order, ClosedTrade};

#[derive(PartialEq, Eq)]
pub enum Side {
    None,
    Long,
    Short,
}

pub struct Portfolio {
    pub balance: f64,
    pub position_size: usize,
    pub avg_entry: f64,
    pub current_side: Side,
    pub realized_pnl: f64,
    pub orders: Vec<Order>,
    pub closed_trades: Vec<ClosedTrade>,
    pub wins: usize,
}

impl Portfolio {
    pub fn new(starting_balance: f64) -> Self {
        Self {
            balance: starting_balance,
            position_size: 0,
            avg_entry: 0.0,
            current_side: Side::None,
            realized_pnl: 0.0,
            orders: Vec::new(),
            closed_trades: Vec::new(),
            wins: 0,
        }
    }

    pub fn update(&mut self, order: &Order) {
        self.orders.push(order.clone());

        match &order.order_type {
            OrderType::Buy => {
                if self.current_side == Side::Short {
                    // close out short position
                    let pnl = (self.avg_entry - order.price) * order.amount as f64;
                    self.closed_trades.push(ClosedTrade::new(pnl));
                    if pnl > 0.0 {
                        self.wins += 1;
                    }
                    self.realized_pnl += pnl;
                    self.balance += pnl;
                    self.position_size -= order.amount;
                }
                
                // go into long position
                self.avg_entry = order.price;
                let cost = order.price * order.amount as f64;
                self.position_size += order.amount;
                self.balance -= cost;

                self.current_side = Side::Long;
            }
            OrderType::Sell => {
                if self.current_side == Side::Long {
                    let pnl = (order.price - self.avg_entry) * order.amount as f64;
                    self.closed_trades.push(ClosedTrade::new(pnl));
                    if pnl > 0.0 {
                        self.wins += 1;
                    }
                    self.realized_pnl += pnl;
                    self.balance += pnl;
                    self.position_size -= order.amount;
                }

                // go into short position
                self.position_size += order.amount;
                self.balance += order.price * order.amount as f64;
                self.current_side = Side::Short;
            }
        }
    }
}

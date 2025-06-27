use crate::{Order, OrderType, SMA};
use crate::Candle;
use crate::strategy::Strategy;

pub struct SMACrossover {
    pub short: SMA,
    pub long: SMA,  
    pub prev_short: Option<f64>,
    pub prev_long: Option<f64>,
    pub in_position: bool,
}

impl SMACrossover {
    pub fn new() -> SMACrossover {
        Self {
            short: SMA::new(10),
            long: SMA::new(50),
            prev_short: None,
            prev_long: None,
            in_position: false,
        }
    }
}

impl Strategy for SMACrossover {
    fn on_candle(&mut self, candle: &Candle) -> Option<Order> {
        let s = self.short.update(candle.close);
        let l = self.long.update(candle.close);

        if let (Some(s), Some(l), Some(ps), Some(pl)) = (s, l, self.prev_short, self.prev_long) {
        
            // short sma crosses up over long sma
            if s > l && ps <= pl && !self.in_position {
                self.in_position = true;
                return Some(Order::new(OrderType::Buy, candle.close, 10))
            }
            // short sma crosses down under long sma
            else if s < l && ps >= pl && self.in_position {
                self.in_position = false;
                return Some(Order::new(OrderType::Sell, candle.close, 10))
            }
        }
        self.prev_short = s;
        self.prev_long = l;

        return None
    }
    
    fn is_in_position(&self) -> bool {
        return self.in_position;
    }
}

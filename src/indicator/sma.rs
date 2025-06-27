use std::collections::VecDeque;

pub struct SMA {
    pub window: VecDeque<f64>,
    pub period: usize,
    pub sum: f64,
}

impl SMA {
    pub fn new(period: usize) -> SMA {
        Self {
            window: VecDeque::with_capacity(period),
            period: period,
            sum: 0.0,
        }
    }

    pub fn update(&mut self, price: f64) -> Option<f64> {
        self.window.push_back(price);
        self.sum += price;
        
        if self.window.len() > self.period {
            if let Some(front) = self.window.pop_front() {
                self.sum -= front;
            }
        }

        if self.window.len() == self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }
} 

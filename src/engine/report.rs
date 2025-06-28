use crate::engine::{Portfolio};

pub struct Report {
    pub final_cash: f64,
    pub final_equity: f64,
    pub total_pnl: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub win_rate: f64,
    pub num_trades: usize,
    pub avg_trade_pnl: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
}

impl Report {
    fn build(portfolio: &Portfolio) -> Self {
        Self {
            final_cash: portfolio.balance,
            final_equity: 0.0,
            total_pnl: portfolio.realized_pnl,
            max_drawdown: 0.0,
            sharpe_ratio: 0.0,
            win_rate: 1.0,
            num_trades: portfolio.trades.len(),
            avg_trade_pnl: portfolio.realized_pnl / portfolio.trades.len() as f64,
            largest_win: 0.0,
            largest_loss: 0.0,
        }
    }
}

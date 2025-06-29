use std::{f64::MAX, fmt};

use crate::{engine::Portfolio, standard_deviation};

pub struct Report {
    pub final_cash: f64,
    pub final_equity: f64,
    pub total_pnl: f64,
    pub sharpe_ratio: f64,
    pub win_rate: f64,
    pub num_trades: usize,
    pub avg_trade_pnl: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
}

impl Report {
    pub fn build(portfolio: &Portfolio) -> Self {
        let mut largest_win = 0.0;
        let mut largest_loss = MAX;

        let total_trades: f64 = portfolio.closed_trades.len() as f64;
        let mut total_return: f64 = 0.0;
        let mut pnl_vec: Vec<f64> = Vec::new();
        
        for i in &portfolio.closed_trades {
            total_return += i.realized_pnl;
            pnl_vec.push(i.realized_pnl);
            if i.realized_pnl > 0.0 && i.realized_pnl > largest_win {
                largest_win = i.realized_pnl;
            }
            if i.realized_pnl < 0.0 && i.realized_pnl < largest_loss {
                largest_loss = i.realized_pnl;
            }
        }

        let mean_return = total_return / total_trades;
        let std_dev = standard_deviation(&pnl_vec);

        let sharpe = if std_dev != 0.0 {
            mean_return / std_dev
        } else {
            0.0
        };

        Self {
            final_cash: portfolio.balance,
            final_equity: portfolio.balance + (portfolio.position_size as f64 * portfolio.avg_entry),
            total_pnl: portfolio.realized_pnl,
            sharpe_ratio: sharpe,
            win_rate: portfolio.wins as f64 / portfolio.closed_trades.len() as f64,
            num_trades: portfolio.closed_trades.len(),
            avg_trade_pnl: portfolio.realized_pnl / portfolio.closed_trades.len() as f64,
            largest_win: largest_win,
            largest_loss: largest_loss,
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", "-".repeat(50))?;
        writeln!(f, "{:<20} {:<10.2}", "Final Cash", self.final_cash).unwrap();
        writeln!(f, "{:<20} {:<10.2}", "Final Equity", self.final_equity).unwrap();
        writeln!(f, "{:<20} {:<10.2}", "Total P/L", self.total_pnl).unwrap();
        writeln!(f, "{:<20} {:<10.2}", "Sharpe Ratio", self.sharpe_ratio).unwrap();
        writeln!(f, "{:<20} {:<10.2} {}", "Win Rate", self.win_rate * 100.0, "%").unwrap();
        writeln!(f, "{:<20} {:<10.2}", "Trades Issued", self.num_trades).unwrap();
        writeln!(f, "{:<20} {:<10.2}", "Average Trade P/L", self.avg_trade_pnl).unwrap();
        writeln!(f, "{:<20} {:<10.2}", "Largest Win", self.largest_win).unwrap();
        writeln!(f, "{:<20} {:<10.2}", "Largest Loss", self.largest_loss).unwrap();
        writeln!(f, "{}", "-".repeat(50))?;

        Ok(())
    }
}

use backtest::{Trader, Report};
use backtest::strategy::SMACrossover;
use backtest::candle_plotter;


fn main() {
    let strat = Box::new( SMACrossover::new() );
    let mut trader = Trader::new(strat, 10_000_000.0);
    if let Err(e) = trader.run() {
        println!("Error occured: {e}");
    }
    println!("realized profit/loss: {}", trader.portfolio.realized_pnl);
    let report = Report::build(&trader.portfolio);
    println!("Final report: {report}");

    println!("Generating plot");
    let path = String::from("bitcoin_1h_1000.csv");
    if let Err(e) = candle_plotter::save_chart(&path, &trader.portfolio.orders) {
        println!("Error generating chart: {e}");
    }

}

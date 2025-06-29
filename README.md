# Rust Backtesting Engine

A simple backtesting engine written in Rust for evaluting different 
trading strategies on historical data. This project was made to learn 
about Rust and trading algorithms.

## Features
- CSV-based historical candlestick data parsing
- Portfolio management with:
    - Final balance and equity
    - Realized PnL
    - Sharpe ratio and win rate reporting
- Candle chart generation with:
    - Bullish/bearish candles
    - Trade markers overlayed on the chart showing Buy and Sell orders
    - SVG chart output for scalable visualization


## Usage
```bash
cargo run --release
```

Make sure that your CSV data is in the correct format:
|index|date|open|high|low|close|volume|
|-----|----|----|----|---|-----|------|
|0|2025-05-13 22:00:00|104342.7|104432.03|104100.0|104128.5|502.72511|


## TODO
- Add CLI arguments to file paths, chart generation and trade parameters
- Add more strategies

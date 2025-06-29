use plotters::prelude::*;
use std::error::Error;
use csv::ReaderBuilder;
use chrono::{ NaiveDateTime, DateTime, Utc, TimeZone };

use std::collections::HashMap;


use crate::{ Candle, Order, OrderType };


pub fn save_chart(path: &String, order_map: &HashMap<usize, Order>) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(path)?;
    let mut candles: Vec<Candle> = Vec::new();

    for result in rdr.deserialize() {
        let record: Candle = result?;
        
        candles.push(record);
    }

    let svg_backend = SVGBackend::new("chart.svg", (1280, 720));
    let root = svg_backend.into_drawing_area();

    root.fill(&WHITE)?;

    let (min_price, max_price) = candles.iter().fold((f64::MAX, f64::MIN), |(min, max), c| {
        (min.min(c.low), max.max(c.high))
    });

    let time_range = 0..candles.len();
    let price_range = min_price..max_price;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Backtesting view", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(time_range.clone(), price_range)?;
    
    chart.configure_mesh().disable_mesh().draw()?;

    // Figure out candle width
    let plot_area = chart.plotting_area();
    let (width, _height) = plot_area.dim_in_pixel();
    let candle_width = width / candles.len() as u32;

    for (i, c) in candles.iter().enumerate() {
        let style = if c.close >= c.open { &GREEN } else { &RED };
        chart.draw_series(vec![
            CandleStick::new(i, c.open, c.high, c.low, c.close, style, style, candle_width)
        ])?;
        if order_map.contains_key(&i) {
            let marker_y = c.high + 10.0;
            let marker_color = if order_map.get(&i).unwrap().order_type == OrderType::Buy { &GREEN } else { &RED };
            chart.draw_series(std::iter::once(
                    Circle::new((i, marker_y), 5, marker_color.filled()),
            ))?;

        }
        
    }

    root.present()?;

    Ok(())
}



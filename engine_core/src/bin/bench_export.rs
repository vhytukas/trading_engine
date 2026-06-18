use engine_core::simulation::{SimConfig, SimOrderKind, Simulator};
use engine_core::{MatchingEngine, Orderbook};
use hdrhistogram::Histogram;
use std::time::Instant;

fn measure_burst(n: u64, lambda: f64) -> Histogram<u64> {
    let config = SimConfig {
        seed: 42,
        mid_price: 10_000,
        price_spread: 50,
        min_qty: 1,
        max_qty: 100,
        market_order_prob: 0.1,
        lambda_per_sec: lambda,
    };

    let mut sim = Simulator::new(config);
    let events: Vec<_> = (0..n).map(|_| sim.next()).collect();

    let mut engine = MatchingEngine::new(Orderbook::new());
    let mut hist = Histogram::<u64>::new_with_bounds(1, 10_000_000, 3).unwrap();

    for ev in &events {
        let start = Instant::now();
        match ev.order.kind {
            SimOrderKind::Limit { price } => {
                engine.place_limit_order(price, ev.order.qty, ev.order.side);
            }
            SimOrderKind::Market => {
                engine.place_market_order(ev.order.qty, ev.order.side);
            }
        }
        let elapsed_ns = start.elapsed().as_nanos() as u64;
        hist.record(elapsed_ns.max(1)).unwrap();
    }

    hist
}

fn print_report(name: &str, hist: &Histogram<u64>) {
    println!("\n=== {name} ===");
    println!("  samples: {}", hist.len());
    println!("  mean:    {} ns", hist.mean().round() as u64);
    println!("  p50:     {} ns", hist.value_at_quantile(0.50));
    println!("  p99:     {} ns", hist.value_at_quantile(0.99));
    println!("  p99.9:   {} ns", hist.value_at_quantile(0.999));
    println!("  p99.99:  {} ns", hist.value_at_quantile(0.9999));
    println!("  max:     {} ns", hist.max());
}
fn main() {
    print_report("burst_n100k_lambda_1k", &measure_burst(100_000, 1_000.0));
}

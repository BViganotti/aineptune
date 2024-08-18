use dashmap::DashMap;
//use num_traits::atomic::AtomicF64;
use atomic_float::AtomicF64;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use warp::Filter;

type TradingData = Arc<DashMap<String, (Vec<f64>, Statistics)>>;

#[derive(Serialize, Deserialize)]
struct BatchToAdd {
    symbol: String,
    values: Vec<f64>,
}

struct Statistics {
    min: AtomicF64,
    max: AtomicF64,
    last: AtomicF64,
    sum: AtomicF64,
    sum_of_squares: AtomicF64,
    count: AtomicUsize,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            min: AtomicF64::new(f64::INFINITY),
            max: AtomicF64::new(f64::NEG_INFINITY),
            last: AtomicF64::new(0.0),
            sum: AtomicF64::new(0.0),
            sum_of_squares: AtomicF64::new(0.0),
            count: AtomicUsize::new(0),
        }
    }
}

impl Statistics {
    fn update(&self, value: f64) {
        self.min.fetch_min(value, Ordering::SeqCst);
        self.max.fetch_max(value, Ordering::SeqCst);
        self.last.store(value, Ordering::SeqCst);
        self.sum.fetch_add(value, Ordering::SeqCst);
        self.sum_of_squares
            .fetch_add(value * value, Ordering::SeqCst);
        self.count.fetch_add(1, Ordering::SeqCst);
    }
}

#[derive(Deserialize)]
struct ToProcess {
    symbol: String,
    k: u32,
}

async fn get_data(
    input_json: ToProcess,
    data: TradingData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let symbol = input_json.symbol;
    let k = input_json.k;

    if let Some(entry) = data.get(&symbol) {
        let (values, stats) = entry.value();
        if values.is_empty() {
            println!("No data available");
            return Ok(warp::reply::json(&"No data available"));
        }

        let n = 10usize.pow(k).min(stats.count.load(Ordering::SeqCst));
        let avg = stats.sum.load(Ordering::SeqCst) / n as f64;
        let var = (stats.sum_of_squares.load(Ordering::SeqCst) / n as f64) - (avg * avg);

        let response = format!(
            "min: {}, max: {}, last: {}, avg: {}, var: {}",
            stats.min.load(Ordering::SeqCst),
            stats.max.load(Ordering::SeqCst),
            stats.last.load(Ordering::SeqCst),
            avg,
            var
        );
        println!("{}", response);

        return Ok(warp::reply::json(&response));
    }

    println!("Symbol not found");
    Ok(warp::reply::json(&"Symbol not found"))
}

async fn post_data(
    item: BatchToAdd,
    data: TradingData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut entry = data.entry(item.symbol.clone()).or_default();
    let (values, stats) = entry.value_mut();

    use std::sync::Mutex;

    let values_mutex = Arc::new(Mutex::new(values));

    item.values.par_iter().for_each(|&value| {
        let mut values = values_mutex.lock().unwrap();
        values.push(value);
        stats.update(value);
    });

    println!("Batch added for symbol: {}", item.symbol);
    Ok(warp::reply::with_status(
        "Batch added!",
        warp::http::StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {
    let trading_data: TradingData = Arc::new(DashMap::new());

    let get_route = warp::path("stats")
        .and(warp::get())
        .and(warp::body::json())
        .and(with_data(trading_data.clone()))
        .and_then(get_data);

    let post_route = warp::path("add_batch")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_data(trading_data.clone()))
        .and_then(post_data);

    let routes = get_route.or(post_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn with_data(
    data: TradingData,
) -> impl Filter<Extract = (TradingData,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data.clone())
}

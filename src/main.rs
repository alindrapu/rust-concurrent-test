use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Order {
    id: u64,
    price: u64,
    is_buy: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Buy(Order); // Highst price first
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Sell(Order); // lowest price first

impl Ord for Buy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .price
            .cmp(&other.0.price)
            .then_with(|| other.0.id.cmp(&self.0.id))
    }
}

impl PartialOrd for Buy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .price
            .cmp(&other.0.price)
            .then_with(|| other.0.id.cmp(&self.0.id))
    }
}

impl PartialOrd for Sell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Order>(64);

    let gen_ = tokio::spawn(async move {
        generate_orders(tx, 1000, 3000).await;
    });

    // in memory queue 
    let mut buy_q: BinaryHeap<Buy> = BinaryHeap::new(); // best buy(higest price)
    let mut sell_q: BinaryHeap<Sell> = BinaryHeap::new(); // best sell(lowest price)


    while let Some(order) = rx.recv().await {
        print_received(&order);

        if order.is_buy {
            // sell
            match sell_q.peek() {
                Some(best_sell) if order.price >= best_sell.0.price => {
                    // if match then sell
                    let matched_sell = sell_q.pop().unwrap().0;
                    print_matched(&order, &matched_sell, order.price);
                }
                _ => {
                    // no match then enqueue
                    buy_q.push(Buy(order));
                }
            }
        } else {
            // buy
            match buy_q.peek() {
                Some(best_buy) if best_buy.0.price >= order.price => {
                    let matched_buy = buy_q.pop().unwrap().0;
                    print_matched(&order, &matched_buy, order.price);
                }
                _ => {
                    sell_q.push(Sell(order));
                }
            }
        }
    }

    let _ = gen_.await;
}

async fn generate_orders(tx: mpsc::Sender<Order>, count: usize, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    for id in 1..=count as u64 {
        let delay_ms = rng.random_range(10..=200);
        sleep(Duration::from_millis(delay_ms)).await;

        // Set random price
        let price: u64 = rng.random_range(10..=150);

        // Set random is_buy
        let is_buy: bool = rng.random_bool(0.5);

        let order = Order { id, price, is_buy };
        if tx.send(order).await.is_err() {
            break;
        }
    }
}

fn print_matched(buy: &Order, sell: &Order, price: u64) {
    println!(
        "Matched: Buy Order {{ id: {} }} Sell Order {{id: {} }} at price {}",
        buy.id, sell.id, price
    );
}

fn print_received(order: &Order) {
    if order.is_buy {
        println!(
            "Received: Buy Order {{ id: {}, price: {} }}",
            order.id, order.price
        );
    } else {
        println!(
            "Received: Sell Order {{ id: {}, price: {} }}",
            order.id, order.price
        );
    }
}

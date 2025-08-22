# Rust Concurrent Programming Test

A simple order matching system implemented in Rust to demonstrate concurrent programming concepts using Tokio.

## Overview

This project simulates a basic order matching system where buy and sell orders are generated asynchronously and matched based on price. It demonstrates several Rust concurrent programming concepts:

- Asynchronous programming with Tokio
- Channel-based communication between tasks
- Priority queues for efficient order matching
- Custom ordering implementations for data structures

## Features

- Asynchronous order generation with random delays
- Price-based order matching algorithm
- Priority queues for buy and sell orders
- Real-time order matching and reporting

## How It Works

The system consists of two main components:

1. **Order Generator**: Asynchronously generates random buy and sell orders with varying prices and sends them through a channel.

2. **Order Matcher**: Receives orders from the channel and attempts to match them with existing orders in the buy and sell queues.

### Order Matching Logic

- Buy orders are matched with the lowest-priced sell order if the buy price is greater than or equal to the sell price.
- Sell orders are matched with the highest-priced buy order if the sell price is less than or equal to the buy price.
- Unmatched orders are added to their respective queues for future matching.

## Running the Project

```bash
# Clone the repository
git clone https://github.com/alindrapu/rust-concurrent-test
cd rust-concurrent-test

# Run the project
cargo run
```

## Example Output

```
Received: Buy Order { id: 1, price: 120 }
Received: Sell Order { id: 2, price: 100 }
Matched: Buy Order { id: 1 } Sell Order {id: 2 } at price 100
Received: Buy Order { id: 3, price: 90 }
```

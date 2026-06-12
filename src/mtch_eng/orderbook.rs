use std::collections::HashMap;

//This will a matching engineering project for the cryptocurrency i am developing
#[derive(Debug)]
pub enum BidOrAsk {
    //specify if it is a buy or a sell order
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order, price: f64) {
        let price_key = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let buy_limit = self.bids.get_mut(&price_key);
                match buy_limit {
                    Some(buy_limit) => {
                        buy_limit.add_order(order);
                    }
                    None => {
                        let mut new_limit = Limit::new(price);
                        new_limit.add_order(order);
                        self.bids.insert(price_key, new_limit);
                    }
                }
            }
            BidOrAsk::Ask => {
                let ask_limit = self.asks.get_mut(&price_key);
                match ask_limit {
                    Some(ask_limit) => {
                        ask_limit.add_order(order);
                    }
                    None => {
                        let mut new_limit = Limit::new(price);
                        new_limit.add_order(order);
                        self.asks.insert(price_key, new_limit);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    //want to ask how much liquidity is sitting at that price level

    pub fn new(price: f64) -> Limit {
        Limit {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    fn total_volume(&self) -> f64 {
        let volume: f64 = self
            .orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap();

        volume //same as saying return the volume
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    pub size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_order_fill_partial() {
        println!("hello world");
        let mut limit = Limit::new(10000.0);
        let buy_limit_order_a = Order::new(BidOrAsk::Ask, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Ask, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 100.0);

        println!("THIS IS {:?}", limit)
    }

    #[test]
    fn limit_order_fill_full() {
        let mut limit = Limit::new(10000.0);
        let buy_limit_order = Order::new(BidOrAsk::Ask, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 0.0);
    }
}

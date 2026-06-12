use super::orderbook::{Order, OrderBook};
use std::collections::HashMap;

//for the crypto BTC USD, the  base is going to be  BTC, and the quote will be USD;
//the quote is the amount of the quote currency that you will pay for the base currency; the base is the amount of  the base currency you will receive for the quote currency

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    //need thing to hold multiple order books bc we will have multiple markets and trading des
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    //the arrow means that 'we will return a blank', so for use pub fn new() -> MatchingEngine
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    //self here needs to be mutable bc we are going to be be modifying self by inserting into its HashMap, this is a chain of edits so self must be mutable
    pub fn add_new_market(&mut self, pair: &TradingPair) {
        //when there is no arrow we are not returning anything; remember that the self must be mutable if we are going to update it
        self.orderbooks.insert(pair.clone(), OrderBook::new()); //in rust we CANNOT use a variable twice
        println!("opening new order book for market: {}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: &TradingPair, //think of the trading pair like an asset
        price: f64,
        order: Order,
    ) -> Result<(), String> {
        //returning a result is the equivalent of returning nothing if it ran fine and Err(String) if there is a specific error
        match self.orderbooks.get_mut(pair) {
            Some(orderbook) => {
                orderbook.add_order(order, price);

                println!("place limit order at the price level {}", price);

                Ok(()) //we can also return a bool like Ok(true) here but we don't need to right here
            }
            None => Err(format!(
                "The order for the given trading pair ({}) doesn't exist rn",
                pair.to_string()
            )),
        }
    }
}

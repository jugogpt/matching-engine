mod mtch_eng;
use mtch_eng::engine::{MatchingEngine, TradingPair};
use mtch_eng::orderbook::{BidOrAsk, Order, OrderBook};

fn main() {
    // if you are going to update a variable in a main class then you need to declare it as mutable

    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 3.0);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.0);

    let mut order_book = OrderBook::new();
    order_book.add_order(buy_order_from_alice, 100.0);
    order_book.add_order(buy_order_from_bob, 100.0);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    order_book.add_order(sell_order, 101.0);

    // a limit is just a unbounded set of these bid/ask orders, must be all bid or all ask
    // in order to print all of the components, use print in this form:
    //println!("{:?}", order_book);
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string()); //trading pair is not mutable bc its impl only has its instantiation
    let fst_pair = TradingPair::new("FST".to_string(), "USD".to_string());
    // open markets for both pairs
    engine.add_new_market(&pair);
    //engine.add_new_market(&fst_pair);

    //IMPORTANT Distinction: a market order executes immediately and takes the form "buy BTC right now at whaever the current price is"
    // while a limit order sits in the orderbook waiting for a market maker and takes the form "buy BTC b ut only if the price drops to $60,000"
    //The think about market orders is that they always must be filled immediately
    //this happens because market orders are filled immediately bc they don't specify a price
    //market orders just buy or sell at the price
    //the last match price between a limit orde rand a market order becomes the new "price" of the asset
    //specificallyt the price of an equity is determined by the "best ask" -- the lowest price any seller is willing to accept; Best bid -- the highest price nay buyer is willing to pay
    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    let sell_order = Order::new(BidOrAsk::Ask, 40.2);
    engine.place_limit_order(&pair, 10.000, buy_order).unwrap(); // this results in a string being printed
                                                                 //engine.place_limit_order(&fst_pair, 10.000, sell_order)
                                                                 //.unwrap();
                                                                 //.unwrap() means "if Ok, continue; if Err, crash with the error message"
}

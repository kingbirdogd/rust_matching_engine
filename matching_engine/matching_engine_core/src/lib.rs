use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Buy = 0,
    Sell = 1,
}

impl Side {
    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::Buy),
            1 => Some(Self::Sell),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputType {
    TradeEvent = 2,
    OrderFullyFilled = 3,
    OrderPartiallyFilled = 4,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AddOrderRequest {
    pub order_id: u64,
    pub side: Side,
    pub quantity: u64,
    pub price: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CancelOrderRequest {
    pub order_id: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutputMessage {
    pub r#type: OutputType,
    pub order_id: u64,
    pub quantity: u64,
    pub price: f64,
}

impl OutputMessage {
    pub fn to_csv(&self) -> String {
        match self.r#type {
            OutputType::TradeEvent => format!("2,{},{}", self.quantity, format_price_value(self.price)),
            OutputType::OrderFullyFilled => format!("3,{}", self.order_id),
            OutputType::OrderPartiallyFilled => format!("4,{},{}", self.order_id, self.quantity),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProcessResult {
    pub outputs: Vec<OutputMessage>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug)]
struct EngineOrder {
    id: u64,
    side: Side,
    remaining: u64,
    price: f64,
}

#[derive(Clone, Copy, Debug)]
struct PriceKey(f64);

impl PriceKey {
    fn value(self) -> f64 {
        self.0
    }
}

impl PartialEq for PriceKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for PriceKey {}

impl PartialOrd for PriceKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriceKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MatchingEngine {
    orders: HashMap<u64, EngineOrder>,
    bids: BTreeMap<PriceKey, VecDeque<u64>>,
    asks: BTreeMap<PriceKey, VecDeque<u64>>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.orders.clear();
        self.bids.clear();
        self.asks.clear();
    }

    pub fn handle_add_request(&mut self, add: AddOrderRequest) -> ProcessResult {
        let mut result = ProcessResult::default();

        if add.order_id == 0 {
            result.errors.push("Invalid AddOrderRequest orderid".to_string());
            return result;
        }

        if add.quantity == 0 {
            result.errors.push("Invalid AddOrderRequest quantity".to_string());
            return result;
        }

        if !add.price.is_finite() || add.price <= 0.0 {
            result.errors.push("Invalid AddOrderRequest price".to_string());
            return result;
        }

        if self.orders.contains_key(&add.order_id) {
            result.errors.push(format!("Duplicate orderid: {}", add.order_id));
            return result;
        }

        let mut incoming_remaining = add.quantity;
        let incoming_id = add.order_id;
        let incoming_side = add.side;
        let incoming_price = add.price;

        if incoming_side == Side::Buy {
            let levels: Vec<_> = self.asks.keys().copied().collect();
            for level in levels {
                if level.value() > incoming_price {
                    break;
                }

                let Some(queue) = self.asks.get_mut(&level) else {
                    continue;
                };

                while !queue.is_empty() && incoming_remaining > 0 {
                    let resting_id = queue.front().copied().unwrap();
                    let Some(resting_order) = self.orders.get_mut(&resting_id) else {
                        queue.pop_front();
                        continue;
                    };

                    let traded = incoming_remaining.min(resting_order.remaining);
                    incoming_remaining -= traded;
                    resting_order.remaining -= traded;
                    let trade_price = resting_order.price;
                    let resting_id_for_message = resting_order.id;
                    let resting_remaining_after = resting_order.remaining;

                    result.outputs.push(OutputMessage {
                        r#type: OutputType::TradeEvent,
                        order_id: 0,
                        quantity: traded,
                        price: trade_price,
                    });

                    result.outputs.push(OutputMessage {
                        r#type: if incoming_remaining == 0 {
                            OutputType::OrderFullyFilled
                        } else {
                            OutputType::OrderPartiallyFilled
                        },
                        order_id: incoming_id,
                        quantity: if incoming_remaining == 0 { 0 } else { incoming_remaining },
                        price: 0.0,
                    });

                    if resting_remaining_after == 0 {
                        queue.pop_front();
                        self.orders.remove(&resting_id_for_message);
                        result.outputs.push(OutputMessage {
                            r#type: OutputType::OrderFullyFilled,
                            order_id: resting_id_for_message,
                            quantity: 0,
                            price: 0.0,
                        });
                    } else {
                        result.outputs.push(OutputMessage {
                            r#type: OutputType::OrderPartiallyFilled,
                            order_id: resting_id_for_message,
                            quantity: resting_remaining_after,
                            price: 0.0,
                        });
                    }
                }

                if self.asks.get(&level).is_some_and(|queue| queue.is_empty()) {
                    self.asks.remove(&level);
                }
            }
        } else {
            let levels: Vec<_> = self.bids.keys().copied().collect();
            for level in levels.into_iter().rev() {
                if level.value() < incoming_price {
                    break;
                }

                let Some(queue) = self.bids.get_mut(&level) else {
                    continue;
                };

                while !queue.is_empty() && incoming_remaining > 0 {
                    let resting_id = queue.front().copied().unwrap();
                    let Some(resting_order) = self.orders.get_mut(&resting_id) else {
                        queue.pop_front();
                        continue;
                    };

                    let traded = incoming_remaining.min(resting_order.remaining);
                    incoming_remaining -= traded;
                    resting_order.remaining -= traded;
                    let trade_price = resting_order.price;
                    let resting_id_for_message = resting_order.id;
                    let resting_remaining_after = resting_order.remaining;

                    result.outputs.push(OutputMessage {
                        r#type: OutputType::TradeEvent,
                        order_id: 0,
                        quantity: traded,
                        price: trade_price,
                    });

                    result.outputs.push(OutputMessage {
                        r#type: if incoming_remaining == 0 {
                            OutputType::OrderFullyFilled
                        } else {
                            OutputType::OrderPartiallyFilled
                        },
                        order_id: incoming_id,
                        quantity: if incoming_remaining == 0 { 0 } else { incoming_remaining },
                        price: 0.0,
                    });

                    if resting_remaining_after == 0 {
                        queue.pop_front();
                        self.orders.remove(&resting_id_for_message);
                        result.outputs.push(OutputMessage {
                            r#type: OutputType::OrderFullyFilled,
                            order_id: resting_id_for_message,
                            quantity: 0,
                            price: 0.0,
                        });
                    } else {
                        result.outputs.push(OutputMessage {
                            r#type: OutputType::OrderPartiallyFilled,
                            order_id: resting_id_for_message,
                            quantity: resting_remaining_after,
                            price: 0.0,
                        });
                    }
                }

                if self.bids.get(&level).is_some_and(|queue| queue.is_empty()) {
                    self.bids.remove(&level);
                }
            }
        }

        if incoming_remaining > 0 {
            self.orders.insert(
                incoming_id,
                EngineOrder {
                    id: incoming_id,
                    side: incoming_side,
                    remaining: incoming_remaining,
                    price: incoming_price,
                },
            );

            match incoming_side {
                Side::Buy => {
                    self.bids.entry(PriceKey(incoming_price)).or_default().push_back(incoming_id);
                }
                Side::Sell => {
                    self.asks.entry(PriceKey(incoming_price)).or_default().push_back(incoming_id);
                }
            }
        }

        result
    }

    pub fn handle_cancel_request(&mut self, request: CancelOrderRequest) -> ProcessResult {
        let mut result = ProcessResult::default();

        if request.order_id == 0 {
            result.errors.push("Invalid CancelOrderRequest orderid".to_string());
            return result;
        }

        let Some(order) = self.orders.get(&request.order_id).cloned() else {
            result.errors.push(format!("Unknown orderid for cancel: {}", request.order_id));
            return result;
        };

        match order.side {
            Side::Buy => {
                let key = PriceKey(order.price);
                if let Some(queue) = self.bids.get_mut(&key) {
                    if let Some(index) = queue.iter().position(|candidate| *candidate == request.order_id) {
                        queue.remove(index);
                    }
                    if queue.is_empty() {
                        self.bids.remove(&key);
                    }
                }
            }
            Side::Sell => {
                let key = PriceKey(order.price);
                if let Some(queue) = self.asks.get_mut(&key) {
                    if let Some(index) = queue.iter().position(|candidate| *candidate == request.order_id) {
                        queue.remove(index);
                    }
                    if queue.is_empty() {
                        self.asks.remove(&key);
                    }
                }
            }
        }

        self.orders.remove(&request.order_id);
        result
    }
}

fn format_price_value(value: f64) -> String {
    let mut text = format!("{value:.18}");
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    if text.is_empty() {
        text.push('0');
    }
    text
}

#[derive(Clone, Debug, Default)]
pub struct LineMatchingEngine {
    engine: MatchingEngine,
}

impl LineMatchingEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.engine.reset();
    }

    pub fn process_line(&mut self, raw_line: &str) -> ProcessResult {
        let mut result = ProcessResult::default();
        let line = trim(raw_line.split("//").next().unwrap_or(raw_line));

        if line.is_empty() {
            return result;
        }

        let fields = split_csv(line);
        if fields.is_empty() || fields[0].is_empty() {
            result.errors.push(format!("Unknown message type: {}", line));
            return result;
        }

        let message_type = match parse_u64(fields[0]) {
            Some(value) => value,
            None => {
                result.errors.push(format!("Unknown message type: {}", fields[0]));
                return result;
            }
        };

        if message_type == 0 {
            if fields.len() != 5 {
                result.errors.push("AddOrderRequest expects 5 fields".to_string());
                return result;
            }

            let order_id = match parse_u64(fields[1]) {
                Some(value) => value,
                None => {
                    result.errors.push("Invalid AddOrderRequest orderid".to_string());
                    return result;
                }
            };

            if order_id == 0 {
                result.errors.push("Invalid AddOrderRequest orderid".to_string());
                return result;
            }

            let side = match parse_u64(fields[2]) {
                Some(value) => match Side::from_u64(value) {
                    Some(side) => side,
                    None => {
                        result.errors.push("Invalid AddOrderRequest side".to_string());
                        return result;
                    }
                },
                None => {
                    result.errors.push("Invalid AddOrderRequest side".to_string());
                    return result;
                }
            };

            let quantity = match parse_u64(fields[3]) {
                Some(value) => value,
                None => {
                    result.errors.push("Invalid AddOrderRequest quantity".to_string());
                    return result;
                }
            };

            if quantity == 0 {
                result.errors.push("Invalid AddOrderRequest quantity".to_string());
                return result;
            }

            let price = match parse_price(fields[4]) {
                Some(value) => value,
                None => {
                    result.errors.push("Invalid AddOrderRequest price".to_string());
                    return result;
                }
            };

            return self.engine.handle_add_request(AddOrderRequest {
                order_id,
                side,
                quantity,
                price,
            });
        }

        if message_type == 1 {
            if fields.len() != 2 {
                result.errors.push("CancelOrderRequest expects 2 fields".to_string());
                return result;
            }

            let order_id = match parse_u64(fields[1]) {
                Some(value) => value,
                None => {
                    result.errors.push("Invalid CancelOrderRequest orderid".to_string());
                    return result;
                }
            };

            if order_id == 0 {
                result.errors.push("Invalid CancelOrderRequest orderid".to_string());
                return result;
            }

            return self.engine.handle_cancel_request(CancelOrderRequest { order_id });
        }

        result.errors.push(format!("Unknown message type: {}", fields[0]));
        result
    }
}

fn trim(value: &str) -> &str {
    value.trim()
}

fn split_csv(line: &str) -> Vec<&str> {
    let mut fields = Vec::new();
    let mut start = 0;
    let bytes = line.as_bytes();
    for (idx, &byte) in bytes.iter().enumerate() {
        if byte == b',' {
            fields.push(trim(&line[start..idx]));
            start = idx + 1;
        }
    }
    fields.push(trim(&line[start..]));
    fields
}

fn parse_u64(text: &str) -> Option<u64> {
    let value = trim(text);
    if value.is_empty() {
        return None;
    }
    value.parse::<u64>().ok()
}

fn parse_price(text: &str) -> Option<f64> {
    let value = trim(text);
    if value.is_empty() {
        return None;
    }
    let parsed = value.parse::<f64>().ok()?;
    if !parsed.is_finite() || parsed <= 0.0 {
        return None;
    }
    Some(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outputs_to_csv(result: &ProcessResult) -> Vec<String> {
        result.outputs.iter().map(|msg| msg.to_csv()).collect()
    }

    #[test]
    fn rejects_invalid_add_requests() {
        let mut engine = MatchingEngine::new();
        assert_eq!(
            engine.handle_add_request(AddOrderRequest { order_id: 0, side: Side::Buy, quantity: 1, price: 100.0 }).errors[0],
            "Invalid AddOrderRequest orderid"
        );
        assert_eq!(
            engine.handle_add_request(AddOrderRequest { order_id: 1, side: Side::Buy, quantity: 0, price: 100.0 }).errors[0],
            "Invalid AddOrderRequest quantity"
        );
        assert_eq!(
            engine.handle_add_request(AddOrderRequest { order_id: 1, side: Side::Buy, quantity: 1, price: 0.0 }).errors[0],
            "Invalid AddOrderRequest price"
        );
    }

    #[test]
    fn rejects_invalid_cancel_requests() {
        let mut engine = MatchingEngine::new();
        assert_eq!(engine.handle_cancel_request(CancelOrderRequest { order_id: 0 }).errors[0], "Invalid CancelOrderRequest orderid");
        assert_eq!(engine.handle_cancel_request(CancelOrderRequest { order_id: 999 }).errors[0], "Unknown orderid for cancel: 999");
    }

    #[test]
    fn detects_duplicate_order_id() {
        let mut engine = MatchingEngine::new();
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 100, side: Side::Buy, quantity: 5, price: 100.0 }).errors.is_empty());
        let dup = engine.handle_add_request(AddOrderRequest { order_id: 100, side: Side::Sell, quantity: 4, price: 99.0 });
        assert_eq!(dup.errors[0], "Duplicate orderid: 100");
    }

    #[test]
    fn matches_at_resting_order_price() {
        let mut engine = MatchingEngine::new();
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 300, side: Side::Sell, quantity: 5, price: 101.0 }).errors.is_empty());
        let buy = engine.handle_add_request(AddOrderRequest { order_id: 301, side: Side::Buy, quantity: 2, price: 105.0 });
        assert!(buy.errors.is_empty());
        assert_eq!(outputs_to_csv(&buy), vec!["2,2,101", "3,301", "4,300,3"]);
    }

    #[test]
    fn price_priority_across_levels() {
        let mut engine = MatchingEngine::new();
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 400, side: Side::Sell, quantity: 4, price: 101.0 }).errors.is_empty());
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 401, side: Side::Sell, quantity: 4, price: 100.0 }).errors.is_empty());
        let buy = engine.handle_add_request(AddOrderRequest { order_id: 402, side: Side::Buy, quantity: 6, price: 101.0 });
        assert!(buy.errors.is_empty());
        assert_eq!(
            outputs_to_csv(&buy),
            vec!["2,4,100", "4,402,2", "3,401", "2,2,101", "3,402", "4,400,2"]
        );
    }

    #[test]
    fn time_priority_within_same_price_level() {
        let mut engine = MatchingEngine::new();
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 500, side: Side::Sell, quantity: 2, price: 100.0 }).errors.is_empty());
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 501, side: Side::Sell, quantity: 2, price: 100.0 }).errors.is_empty());
        let buy = engine.handle_add_request(AddOrderRequest { order_id: 502, side: Side::Buy, quantity: 3, price: 100.0 });
        assert!(buy.errors.is_empty());
        assert_eq!(
            outputs_to_csv(&buy),
            vec!["2,2,100", "4,502,1", "3,500", "2,1,100", "3,502", "4,501,1"]
        );
    }

    #[test]
    fn sell_aggressor_matches_best_bid_first() {
        let mut engine = MatchingEngine::new();
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 600, side: Side::Buy, quantity: 3, price: 99.0 }).errors.is_empty());
        assert!(engine.handle_add_request(AddOrderRequest { order_id: 601, side: Side::Buy, quantity: 2, price: 100.0 }).errors.is_empty());
        let sell = engine.handle_add_request(AddOrderRequest { order_id: 602, side: Side::Sell, quantity: 4, price: 99.0 });
        assert!(sell.errors.is_empty());
        assert_eq!(
            outputs_to_csv(&sell),
            vec!["2,2,100", "4,602,2", "3,601", "2,2,99", "3,602", "4,600,1"]
        );
    }

    #[test]
    fn parses_and_routes_line_messages() {
        let mut engine = LineMatchingEngine::new();
        assert!(engine.process_line("0,100,1,5,101 // resting ask").errors.is_empty());
        let buy = engine.process_line("0,101,0,2,105");
        assert!(buy.errors.is_empty());
        assert_eq!(outputs_to_csv(&buy), vec!["2,2,101", "3,101", "4,100,3"]);
    }

    #[test]
    fn line_matches_invalid_message_types() {
        let mut engine = LineMatchingEngine::new();
        let bad_text = engine.process_line("foo,1,2");
        assert_eq!(bad_text.errors[0], "Unknown message type: foo");

        let bad_num = engine.process_line("9,1,2");
        assert_eq!(bad_num.errors[0], "Unknown message type: 9");
    }
}

use models::*;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub order_type: TradeType,
    pub qty: f64,
    pub price: f64,
}

impl Order {
    pub fn to_trade(&self) -> Trade {
        Trade {
            cost: self.price,
            qty: self.qty,
            buy: self.order_type.buy(),
            // order_type: TradeType,
        }
    }
}

// reduce all orders with same price together (technically different orders from the order book)
pub fn group_by_price(orders: Vec<Order>) -> Vec<Order> {
    let mut grouped_orders = Vec::new();
    let first_order = orders.clone();
    let first_order = first_order.first();
    let orders = orders.clone().split_off(1);
    // println!("orders {:?}", orders.clone());

    // todo: rewrite using peek
    if let Some(first_order) = first_order {
        let mut current_order = first_order.clone();
        // println!("first {:?}", first_order.clone());
        // println!("orders {:?}", orders.clone());

        if !orders.is_empty() {
            for order in orders {
                // println!("checking {}={}, {}={}", order.price, current_order.price, order.order_type, current_order.order_type);
                if order.price == current_order.price && order.order_type == current_order.order_type {
                    // println!("same {:?}", current_order.clone());
                    current_order.qty += order.qty;
                } else {
                    // println!("different {:?}", current_order.clone());
                    grouped_orders.push(current_order.clone());
                    current_order = order.clone();
                }
            }
        }

        grouped_orders.push(current_order.clone());
    }
    // grouped_orders.into_iter().rev().collect()
    grouped_orders
}

#[cfg(test)]
fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
    Order{ id: "".to_string(), symbol: "".to_string(), order_type: order_type, qty: qty, price: price }
}

#[test]
fn test_group_by_price_1() {
    fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
        Order{ id: "".to_string(), symbol: "".to_string(), order_type: order_type, qty: qty, price: price }
    }

    let orders = group_by_price(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
    ]);

    assert_eq!(orders.len(), 1);

    let order = orders.first().unwrap();
    assert_eq!(order.qty, 1.0);
    assert_eq!(order.price, 100.0);
}

#[test]
fn test_group_by_price_2() {
    let orders = group_by_price(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Buy, 1.0, 100.0),
    ]);

    assert_eq!(orders.len(), 1);

    let order = orders.first().unwrap();
    assert_eq!(order.qty, 2.0);
    assert_eq!(order.price, 100.0);
}

#[test]
fn test_group_by_price_3() {
    let orders = group_by_price(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Sell, 1.0, 100.0),
    ]);

    println!("{:?}", orders);

    assert_eq!(orders.len(), 2);

    let order = orders.first().unwrap();
    assert_eq!(order.qty, 2.0);
    assert_eq!(order.price, 100.0);
    assert_eq!(order.order_type, TradeType::Buy);

    let order = orders.last().unwrap();
    assert_eq!(order.qty, 1.0);
    assert_eq!(order.price, 100.0);
    assert_eq!(order.order_type, TradeType::Sell);
}

#[test]
fn test_group_by_price_4() {
    let orders = group_by_price(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Buy, 1.0, 200.0),
    ]);

    assert_eq!(orders.len(), 2);

    let order = orders.first().unwrap();
    assert_eq!(order.qty, 1.0);
    assert_eq!(order.price, 100.0);

    let order = orders.last().unwrap();
    assert_eq!(order.qty, 1.0);
    assert_eq!(order.price, 200.0);
}

// group/average orders into a grouped vector by buy/sell type
// used by average orders - not really too useful directly.
pub fn group_orders(orders: Vec<Order>) -> Vec<Vec<Order>> {
    let mut grouped_orders = Vec::new();
    let mut current_order_group = Vec::new();

    if let Some(first_order) = orders.first() {
        let mut current_order_type = first_order.order_type;

        for order in orders.clone() {
            if order.order_type != current_order_type {
                grouped_orders.push(current_order_group.clone());
                current_order_group = Vec::new();
                current_order_type = order.order_type;
            }
            current_order_group.push(order);
        }
        grouped_orders.push(current_order_group);
    }
    grouped_orders
}

pub fn average_orders(orders: Vec<Order>) -> Vec<Order> {
    group_orders(orders).iter().map(|order_group| {
        average_order(order_group.to_vec())
    }).collect()
}

pub fn average_order(orders: Vec<Order>) -> Order {
    let mut first_order = orders.first().unwrap().clone();
    let total_qty = orders.iter().map(|o| o.qty).sum();
    let total_price = orders.iter().map(|o| o.qty * o.price).sum::<f64>();
    let average_price = total_price / total_qty as f64;

    // println!("total qty: {}, total price: {}, average_price: {}", total_qty, total_price, average_price);

    first_order.price   = average_price;
    first_order.qty     = total_qty;

    first_order
}

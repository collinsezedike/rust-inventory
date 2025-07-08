mod order;
mod store;
mod product;

use store::*;
use order::*;

fn main() {
    let mut store: Store = Store::new();

    let products_to_add = [
        ("MacBook Pro", 1999.99, 8),
        ("AirPods", 199.99, 3),
        ("iPhone 14", 899.99, 0),
    ];

    for product in products_to_add {
        store.add_product(product.0, product.1, product.2);
    }

    store.list_products();

    let orders_to_place = [
        ("Alice Johnson", 1, 2),
        ("Bob Smith", 2, 5),
        ("Charlie Daniels", 3, 1),
        ("James Carnage", 10000, 1),
        ("John Snow", 2, 1),
    ];

    for order in orders_to_place {
        store.place_order(order.0, order.1, order.2);
    }
    store.list_orders();

    store.update_order_status(101, OrderStatus::Shipped);
    store.list_orders();

    store.update_order_status(101, OrderStatus::Delivered);
    store.list_orders();

    store.list_products();

    store.update_payment_status(101, PaymentStatus::Paid);
    store.list_orders();

    store.update_payment_status(105, PaymentStatus::Refunded);
    store.list_orders();
}

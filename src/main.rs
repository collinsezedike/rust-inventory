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
        ("Alice Johnson", 1, 2, "Shipped"),
        ("Bob Smith", 2, 5, "Cancelled"),
        ("Charlie Daniels", 3, 1, "Pending"),
        ("James Carnage", 102, 1, "Pending"),
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
}

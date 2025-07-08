use crate::product::*;
use crate::order::*;

pub struct Store {
    pub products: Vec<Product>,
    pub orders: Vec<Order>,
    pub next_order_id: u32,
    pub next_product_id: u32,
}

impl Store {
    pub fn new() -> Self {
        let products = Vec::new();
        let orders = Vec::new();
        let next_order_id = 101;
        let next_product_id = 1;
        Store { products, orders, next_order_id, next_product_id }
    }

    pub fn add_product(&mut self, name: &str, price: f64, stock: u32) {
        let new_product = Product {
            id: self.next_product_id,
            name: name.to_string(),
            price,
            stock,
        };
        self.products.push(new_product);
        println!(
            "LOG: successfully added Product {:?}: {} to the store",
            self.next_product_id,
            name
        );
        self.next_product_id += 1;
    }

    pub fn place_order(&mut self, customer_name: &str, product_id: u32, quantity: u32) {
        let new_order = Order {
            id: self.next_order_id,
            customer_name: customer_name.to_string(),
            product_id,
            quantity,
            status: OrderStatus::Pending,
        };
        self.orders.push(new_order);
        println!("LOG: successfully placed Order {:?} to the store", self.next_order_id);
        self.next_order_id += 1;
    }

    pub fn update_order_status(&mut self, order_id: u32, new_status: OrderStatus) {
        for order in &mut self.orders {
            if order.id == order_id {
                order.status = new_status;
                println!("LOG: successfully updated Order {:?} status", order_id);
                break;
            }
        }
    }
    pub fn list_products(&self) {
        println!("\n========== STORE PRODUCT LIST ==========\n");
        for product in &self.products {
            println!(
                "Product ID: {:?}\nName: {}\nPrice: {:?}\nStock: {:?}\n",
                product.id,
                product.name,
                product.price,
                product.stock
            );
        }
    }
    pub fn list_orders(&self) {
        println!("========== STORE ORDER LIST ==========\n");
        for order in &self.orders {
            println!(
                "Order ID: {:?}\nCustomer name: {}\nProduct ID: {:?}\nQty: {:?}\n",
                order.id,
                order.customer_name,
                order.product_id,
                order.quantity
            );
            order.print_status();
        }
    }
}

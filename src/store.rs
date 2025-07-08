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

    // // My approach
    fn auto_cancel_invalid_orders(&mut self) {
        let mut found_product_id = false;
        for order in &mut self.orders {
            for product in &mut self.products {
                if order.product_id == product.id {
                    found_product_id = true;
                    if !product.is_in_stock(order.quantity) {
                        order.cancel_order();
                    } else {
                        let _ = product.reduce_stock(order.quantity);
                    }
                    break;
                }
            }

            if !found_product_id || order.is_paid() {
                order.cancel_order();
            }
        }
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
            customer: Customer { name: customer_name.to_string() },
            product_id,
            quantity,
            status: OrderStatus::Pending,
            payment: PaymentStatus::Unpaid,
        };
        self.orders.push(new_order);
        self.auto_cancel_invalid_orders();
        println!("LOG: successfully placed Order {:?} to the store", self.next_order_id);
        self.next_order_id += 1;
    }

    pub fn update_order_status(&mut self, order_id: u32, new_status: OrderStatus) {
        for order in &mut self.orders {
            if order.id == order_id {
                order.update_status(new_status);
                println!("LOG: successfully updated Order {:?} status", order_id);
                break;
            }
        }
    }

    pub fn update_payment_status(&mut self, order_id: u32, new_payment: PaymentStatus) {
        for order in &mut self.orders {
            if order.id == order_id {
                order.update_payment(new_payment);
                println!("LOG: successfully updated Order {:?} payment", order_id);
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
        println!("\n========== STORE ORDER LIST ==========\n");
        for order in &self.orders {
            println!(
                "Order ID: {:?}\nCustomer: {}\nProduct ID: {:?}\nQty: {:?}",
                order.id,
                order.customer.name,
                order.product_id,
                order.quantity
            );
            order.print_status();
            order.print_payment();
        }
    }

    pub fn search_product(&self, product_name: &str) {
        for product in &self.products {
            if product_name.to_string() == product.name {
                println!("\nFound one product with name: '{}' ", product_name);
                println!(
                    "{{ Product ID: {:?}, Name: {}, Price: {:?}, Stock: {:?} }}",
                    product.id,
                    product.name,
                    product.price,
                    product.stock
                );
                return;
            }
        }
        println!("\n404! No product found with name: '{}'", product_name);
    }
}

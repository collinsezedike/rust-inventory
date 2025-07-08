pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

pub struct Order {
    pub id: u32,
    pub customer_name: String,
    pub product_id: u32,
    pub quantity: u32,
    pub status: OrderStatus,
}

impl Order {
    pub fn update_status(&mut self, new_status: OrderStatus) {
        self.status = new_status;
    }
    pub fn print_status(&self) {
        match self.status {
            OrderStatus::Pending => println!("Order is pending"),
            OrderStatus::Shipped => println!("Order has been shipped"),
            OrderStatus::Delivered => println!("Order has been delivered"),
            OrderStatus::Cancelled => println!("Order has been cancelled"),
        };
    }
}

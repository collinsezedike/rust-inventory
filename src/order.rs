pub struct Customer {
    pub name: String,
}

pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

pub struct Order {
    pub id: u32,
    pub customer: Customer,
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
            OrderStatus::Pending => println!("Status: Pending\n"),
            OrderStatus::Shipped => println!("Status: Shipped\n"),
            OrderStatus::Delivered => println!("Status: Delivered\n"),
            OrderStatus::Cancelled => println!("Status: Cancelled\n"),
        };
    }
}

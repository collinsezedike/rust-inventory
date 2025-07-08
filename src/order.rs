pub struct Customer {
    pub name: String,
}

pub enum PaymentStatus {
    Paid,
    Unpaid,
    Refunded,
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
    pub payment: PaymentStatus,
}

impl Order {
    pub fn update_status(&mut self, new_status: OrderStatus) {
        self.status = new_status;
    }

    pub fn update_payment(&mut self, new_payment: PaymentStatus) {
        self.payment = new_payment;
    }

    pub fn is_paid(&self) -> bool {
        match self.payment {
            PaymentStatus::Paid => true,
            _ => false,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        match self.status {
            OrderStatus::Cancelled => true,
            _ => false,
        }
    }

    pub fn cancel_order(&mut self) {
        if !self.is_cancelled() {
            self.update_status(OrderStatus::Cancelled);
        }

        if self.is_paid() {
            self.update_payment(PaymentStatus::Refunded);
        }
    }

    pub fn print_status(&self) {
        match self.status {
            OrderStatus::Pending => println!("Status: Pending"),
            OrderStatus::Shipped => println!("Status: Shipped"),
            OrderStatus::Delivered => println!("Status: Delivered"),
            OrderStatus::Cancelled => println!("Status: Cancelled"),
        };
    }

    pub fn print_payment(&self) {
        match self.payment {
            PaymentStatus::Paid => println!("Payment: Paid\n"),
            PaymentStatus::Unpaid => println!("Payment: Unpaid\n"),
            PaymentStatus::Refunded => println!("Payment: Refunded\n"),
        };
    }
}

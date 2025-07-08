pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub stock: u32,
}

impl Product {
    // fn generate_product_id() {}

    // pub fn new(name: String, price: f64, stock: u32) -> Self {
    //     let id = Product::generate_product_id()
    //     Product { id, name, price, stock }
    // }

    pub fn is_in_stock(&self, quantity: u32) -> bool {
        self.stock > 0 && self.stock >= quantity
    }

    pub fn reduce_stock(&mut self, quantity: u32) -> Result<(), String> {
        match self.is_in_stock(quantity) {
            true => {
                self.stock -= quantity;
                Ok(())
            }
            false => Err(String::from("There isn't enough stock")),
        }
    }
}

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self::validate_product_name(&product_name);
        Self::validate_quantity(&quantity);
        Self::validate_unit_price(&unit_price);
        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    fn validate_product_name(product_name: &String) {
        if product_name.is_empty() || product_name.len() > 300 {
            panic!("The product name can't be empty and it can't be longer than 300 bytes.");
        }
    }

    fn validate_quantity(quantity: &u32) {
        if *quantity == 0 {
            panic!("The quantity must be strictly greater than zero.");
        }
    }

    fn validate_unit_price(unit_price: &u32) {
        if *unit_price == 0 {
            panic!("The unit price is in cents and must be strictly greater than zero")
        }
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::validate_product_name(&product_name);

        self.product_name = product_name
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        Self::validate_quantity(&quantity);

        self.quantity = quantity
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        Self::validate_unit_price(&unit_price);

        self.unit_price = unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}

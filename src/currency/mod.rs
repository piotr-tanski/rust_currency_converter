pub struct Currency {
    pub code: String,
}

impl Currency {
    pub fn new(code: String) -> Currency {
        Currency {
            code
        }
    }
}

pub struct PLN {
    pub amount: i32,
}

impl PLN {
    pub fn new(amount: i32) -> PLN {
        PLN {
            amount,
        }
    }
}

pub fn convert_to(pln: &PLN, rate: f32) -> f32 {
    pln.amount as f32 / rate
}
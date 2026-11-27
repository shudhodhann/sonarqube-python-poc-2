
pub fn calculate_discount(price: f64, discount: f64) -> f64 {
    price - (price * discount / 100.0)
}

pub fn copy_values(values: &[i32]) -> Vec<i32> {
    values.to_vec()
}

pub fn is_valid(value: bool) -> bool {
    value
}

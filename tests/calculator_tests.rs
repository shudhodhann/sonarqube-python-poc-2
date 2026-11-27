use rust_sonar_alert_poc::{
    calculate_discount,
    copy_values,
    is_valid,
};

#[test]
fn test_calculate_discount() {
    assert_eq!(calculate_discount(100.0, 10.0), 90.0);
}

#[test]
fn test_copy_values() {
    assert_eq!(copy_values(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_is_valid() {
    assert!(is_valid(true));
}
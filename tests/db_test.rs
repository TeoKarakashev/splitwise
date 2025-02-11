use splitwise::db::{Database, Payment};

#[test]
fn test_add_user() {
    let mut db = Database::new().unwrap();
    let user_id = db.add_user("Alice").expect("Failed to add user");

    assert!(user_id > 0);
    assert_eq!(db.get_balances_with_users().len(), 1);
}

#[test]
fn test_add_payment() {
    let mut db = Database::new().unwrap();
    let user_id = db.add_user("Bob").unwrap();
    db.add_payment("Lunch", 20.0, user_id).unwrap();

    let payments = db.get_all_payments();
    assert_eq!(payments.len(), 1);
    assert_eq!(payments[0].description, "Lunch");
    assert_eq!(payments[0].amount, 20.0);
}

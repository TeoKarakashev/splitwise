use splitwise::app::{App, Tab};
use splitwise::messages::Message;

#[test]
fn test_app_initial_state() {
    let app = App::new();
    assert!(app.payments.is_empty());
    assert!(app.balances.is_empty());
    assert!(app.split_with_input.is_empty());
    assert!(matches!(app.active_tab, Tab::Payments));
}

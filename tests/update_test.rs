use splitwise::app::{App, Tab};
use splitwise::messages::Message;
use splitwise::update::update;

#[test]
fn test_input_changes() {
    let mut app = App::new();
    
    // Simulate user input
    update(&mut app, Message::SplitWithInputChanged("Alice".to_string()));
    assert_eq!(app.split_with_input, "Alice");

    update(&mut app, Message::AmountInputChanged("50".to_string()));
    assert_eq!(app.amount_input, "50");

    update(&mut app, Message::DescriptionInputChanged("Dinner".to_string()));
    assert_eq!(app.description_input, "Dinner");
}

#[test]
fn test_switch_tabs() {
    let mut app = App::new();
    update(&mut app, Message::SwitchToHistory);
    assert!(matches!(app.active_tab, Tab::History));
}

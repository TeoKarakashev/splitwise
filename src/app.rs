use iced::{Element, Sandbox};

use crate::{db::{Database, Payment}, update::update, view::view, messages::Message};

pub enum Tab {
    Payments,
    SettleUp(String), 
    History,
}

pub struct App {
    pub db: Database,
    pub split_with_input: String,
    pub amount_input: String,
    pub description_input: String,
    pub settle_amount_input: String,
    pub settle_with: Option<String>,
    pub payments: Vec<Payment>,
    pub balances: Vec<(String, f64)>,
    pub active_tab: Tab,
}

impl App {
    pub fn new() -> Self {
        let db = Database::new().expect("Failed to initialize database");
        let payments = db.get_all_payments().unwrap_or_default();
        let balances = db.get_balances_with_users().unwrap_or_default();
        
        Self {
            db,
            split_with_input: String::new(),
            amount_input: String::new(),
            description_input: String::new(),
            settle_amount_input: String::new(),
            settle_with: None,
            payments,
            balances,
            active_tab: Tab::Payments,
        }
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::new() 
    }

    fn title(&self) -> String {
        String::from("Splitwise")
    }

    fn update(&mut self, message: Message) {
        update(self, message);
    }

    fn view(&self) -> Element<'_, Self::Message> {
        view(self)
    }
}
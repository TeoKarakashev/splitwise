use crate::{app::App, messages::Message, app::Tab}; // Updated imports

pub fn update(app: &mut App, message: Message) {
    match message {
        Message::SplitWithInputChanged(value) => {
            app.split_with_input = value;
        }
        Message::AmountInputChanged(value) => {
            app.amount_input = value;
        }
        Message::DescriptionInputChanged(value) => {
            app.description_input = value;
        }
        Message::SplitPayment => {
            if !app.split_with_input.is_empty() && !app.amount_input.is_empty() {
                let friend_id = app
                    .db
                    .add_user(&app.split_with_input)
                    .expect("Failed to add friend");
                let amount = app.amount_input.parse::<f64>().unwrap_or(0.0);

                let amount_per_person = amount / 2.0; // Split the amount equally

                app.db
                    .add_payment(&app.description_input, amount_per_person, friend_id)
                    .expect("Failed to add split payment");

                app.payments = app.db.get_all_payments().unwrap_or_default();
                app.balances = app.db.get_balances_with_users().unwrap_or_default();

                app.split_with_input.clear();
                app.amount_input.clear();
                app.description_input.clear();
            }
        }
        Message::SettleUp(user) => {
            app.settle_with = Some(user.clone());
            app.settle_amount_input.clear();
            app.active_tab = Tab::SettleUp(user);
        }
        Message::SettleAmountChanged(value) => {
            app.settle_amount_input = value;
        }
        Message::ConfirmSettleUp => {
            if let Some(user) = &app.settle_with {
                if !app.settle_amount_input.is_empty() {
                    let amount = app.settle_amount_input.parse::<f64>().unwrap_or(0.0);
                    app.db.settle_payment(user, amount).expect("Failed to settle payment");

                    app.payments = app.db.get_all_payments().unwrap_or_default();
                    app.balances = app.db.get_balances_with_users().unwrap_or_default();
                }
                app.settle_with = None;
                app.settle_amount_input.clear();
                app.active_tab = Tab::Payments;
            }
        }
        Message::SwitchToPayments => app.active_tab = Tab::Payments,
        Message::SwitchToHistory => app.active_tab = Tab::History,
    }
}

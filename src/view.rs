use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{theme, Alignment, Element, Theme, Color};

use crate::{messages::Message, app::App, app::Tab};

const GREEN: Color = Color::from_rgb(0.1, 0.7, 0.3);
const BLACK: Color = Color::BLACK;
const WHITE: Color = Color::WHITE;

fn green_button(label: &str, msg: Message) -> button::Button<Message> {
    button(text(label).size(18).style(theme::Text::Color(WHITE)))
        .padding(12)
        .style(theme::Button::Custom(Box::new(GreenButton)))
        .on_press(msg)
}

struct GreenButton;
impl button::StyleSheet for GreenButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(GREEN.into()),
            border_radius: 10.0.into(),
            text_color: WHITE,
            ..Default::default()
        }
    }
}

pub fn view(app: &App) -> Element<Message> {
    let tab_selector = row![
        green_button("Payments", Message::SwitchToPayments),
        green_button("History", Message::SwitchToHistory)
    ]
    .spacing(20)
    .align_items(Alignment::Center);

    let content = match &app.active_tab {
        Tab::Payments => {
            let balance_list = app.balances.iter().fold(
                column!().spacing(10),
                |col, (user, balance)| {
                    let status = if *balance > 0.0 {
                        format!("owes you {:.2}", balance)
                    } else if *balance < 0.0 {
                        format!("you owe {:.2}", -balance)
                    } else {
                        "settled".to_string()
                    };

                    col.push(
                        row![
                            text(format!("{}: {}", user, status))
                                .size(18)
                                .style(theme::Text::Color(BLACK))
                                .horizontal_alignment(Horizontal::Left),
                            green_button("Settle Up", Message::SettleUp(user.clone()))
                        ]
                        .spacing(10)
                    )
                },
            );

            column![
                text("Split a Payment")
                    .size(24)
                    .style(theme::Text::Color(GREEN))
                    .horizontal_alignment(Horizontal::Center),
                text_input("Name", &app.split_with_input)
                    .on_input(Message::SplitWithInputChanged)
                    .padding(10),
                text_input("Amount", &app.amount_input)
                    .on_input(Message::AmountInputChanged)
                    .padding(10),
                text_input("Description", &app.description_input)
                    .on_input(Message::DescriptionInputChanged)
                    .padding(10),
                green_button("Split Payment", Message::SplitPayment),
                text("Balances:")
                    .size(22)
                    .style(theme::Text::Color(BLACK))
                    .horizontal_alignment(Horizontal::Center),
                balance_list
            ]
            .padding(20)
            .spacing(15)
        }
        Tab::SettleUp(user) => column![
            text(format!("Settle Up with {}", user))
                .size(24)
                .style(theme::Text::Color(GREEN)),
            text_input("Amount to settle", &app.settle_amount_input)
                .on_input(Message::SettleAmountChanged)
                .padding(10),
            green_button("Confirm", Message::ConfirmSettleUp),
            button(text("Cancel").size(18).style(theme::Text::Color(WHITE)))
                .padding(12)
                .style(theme::Button::Custom(Box::new(GreenButton)))
                .on_press(Message::SwitchToPayments)
        ]
        .spacing(20)
        .padding(20),
        Tab::History => {
            let history = app.payments.iter().fold(
                column!().spacing(10),
                |col, payment| {
                    col.push(
                        text(format!(
                            "{}: {} ({})",
                            payment.description, payment.amount, payment.payee_name
                        ))
                        .size(18)
                        .style(theme::Text::Color(BLACK))
                    )
                },
            );

            column![
                text("History")
                    .size(24)
                    .style(theme::Text::Color(GREEN))
                    .horizontal_alignment(Horizontal::Center),
                history
            ]
            .spacing(20)
            .padding(20)
        }
    };

    scrollable(
        container(column![tab_selector, content])
            .padding(20)
            .center_x()
    )
    .into()
}

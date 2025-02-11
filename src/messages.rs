#[derive(Debug, Clone)]
pub enum Message {
    SplitWithInputChanged(String),
    AmountInputChanged(String),
    DescriptionInputChanged(String),
    SplitPayment,
    SettleUp(String),
    SettleAmountChanged(String),
    ConfirmSettleUp,
    SwitchToPayments,
    SwitchToHistory,
}

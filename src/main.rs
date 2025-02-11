mod app;
mod db;
mod view;
mod update;
mod messages;

use iced::{Settings, Application};  
use app::App;

fn main() -> iced::Result {
    App::run(Settings::default()) 
}
